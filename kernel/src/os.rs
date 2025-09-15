#![allow(clippy::empty_loop)]

use core::ptr::{read_volatile, write_volatile};
use cortex_m_rt::{exception};
use crate::os_config::*;
use crate::systick::{SysTick};
use cortex_m::interrupt;

pub const CORE_CLOCK_MHZ: u32 = 16; 

/// Addresses for System Control Block ICSR register (PendSV set-pending bit)
const SCB_ICSR: *mut u32 = 0xE000_ED04 as *mut u32;

// == External assembly symbols (implemented in context_switch.s) ==
unsafe extern "C" {
    fn init_scheduler_stack(top_of_stack: u32);
    fn switch_sp_to_psp();    
    fn PendSV_Handler();
}

/// Current task index and global tick (static mut; accessed under critical sections)
static mut CURRENT_TASK_IDX: usize = 0;
static mut GLOBAL_TICK_COUNT: u32 = 0;

// ---------- Low-level helpers (called from assembly) ----------

#[unsafe(no_mangle)]
pub extern "C" fn get_psp_value() -> u32 {
    interrupt::free(|_| unsafe { TASKS[CURRENT_TASK_IDX].psp_value })
}

#[unsafe(no_mangle)]
pub extern "C" fn save_psp_value(psp: u32) {
    interrupt::free(|_| unsafe {
        TASKS[CURRENT_TASK_IDX].psp_value = psp;
    });
}

// SAFETY: called from PendSV assembly; symbol must be unmangled and C ABI.
#[unsafe(no_mangle)]
pub extern "C" fn update_to_next_task() {
    unsafe {
        let n = MAX_TASK;
        let mut next: usize = 0;        // idle fallback
        let mut best: usize = usize::MAX;

        match SCHEDULER_MODE {
            SchedulerMode::RoundRobin => {
                let mut i = (CURRENT_TASK_IDX + 1) % n;
                for _ in 0..n {
                    if TASKS[i].current_state == TASK_READY_STATE {
                        next = i;
                        break;
                    }
                    i = (i + 1) % n;
                }
            }

            SchedulerMode::Priority => {
                let mut i = (CURRENT_TASK_IDX + 1) % n;
                for _ in 0..n {
                    if TASKS[i].current_state == TASK_READY_STATE {
                        let p = TASKS[i].priority as usize;
                        if p < best {
                            best = p;
                            next = i;
                            // once we pick a new best, we stick with the first one we see
                        }
                    }
                    i = (i + 1) % n;
                }
            }
        }

        CURRENT_TASK_IDX = next;
    }
}





/// Trigger a PendSV to request a context switch.
pub fn schedule() {
    unsafe {
        core::ptr::write_volatile(SCB_ICSR, 1 << 28);
    }
}

#[exception]
fn SysTick() {    
    schedule();
}

#[exception]
fn PendSV() {
    unsafe {
        PendSV_Handler();
    }
}

/// Initializes the process stack for all tasks in `TASKS`.
///
/// # Safety
/// - This function writes directly to raw stack memory.
/// - Caller must ensure:
///   1. The `TASKS` array is valid and fully initialized with correct task handlers.
///   2. The `task_stack_start(i)` returns a valid memory region for each task stack.
///   3. No other code is accessing or modifying these stacks while this runs.
unsafe fn init_task_stack() {
    #[allow(clippy::needless_range_loop)] 
    for i in 0..MAX_TASK {
        unsafe {
            // Get starting PSP for this task
            let mut p = task_stack_start(i) as *mut u32;

            // xPSR with Thumb bit set
            p = p.offset(-1);
            p.write_volatile(DUMMY_XPSR);

            // PC = task entry
            p = p.offset(-1);
            p.write_volatile(TASKS[i].task_handler as usize as u32);

            // LR = return to Thread mode using PSP
            p = p.offset(-1);
            p.write_volatile(0xFFFFFFF9u32); // Thread mode, PSP, no FPU

            // R12, R3, R2, R1, R0
            for _ in 0..5 {
                p = p.offset(-1);
                p.write_volatile(0);
            }
            // R4-R11
            for _ in 0..8 {
                p = p.offset(-1);
                p.write_volatile(0);
            }
            // Save the new PSP value into the TCB
            TASKS[i].psp_value = p as u32;
        }
    }
}

/// Initialize the scheduler: setup task stacks and scheduler MSP stack.
/// Call this once before starting the scheduler.
pub fn scheduler_init() {
    unsafe {
        init_scheduler_stack(scheduler_stack_start());

        // SHPR3 base
        let shpr3 = 0xE000_ED20 as *mut u32;

        // Clear both fields (PendSV [23:16], SysTick [31:24]) then set:
        //   PendSV  = 0xFF (lowest)
        //   SysTick = 0xF0 (low, but above PendSV)
        let mut v = core::ptr::read_volatile(shpr3);
        v &= !((0xFFu32 << 16) | (0xFFu32 << 24)); // clear PendSV & SysTick fields
        v |= (0xFFu32 << 16) | (0xF0u32 << 24);    // set PendSV, SysTick
        core::ptr::write_volatile(shpr3, v);

        // (Optional) byte-wise form—often clearer:
        // *(0xE000_ED22 as *mut u8) = 0xFF; // PendSV
        // *(0xE000_ED23 as *mut u8) = 0xF0; // SysTick

        // If you keep FP enabled, this disables lazy stacking (ASPEN=1, LSPEN=0).
        let fpccr = 0xE000_EF34 as *mut u32;
        let vv = core::ptr::read_volatile(fpccr);
        core::ptr::write_volatile(fpccr, (vv | (1 << 31)) & !(1 << 30));

        let mut systick = SysTick::take().expect("Failed to take SysTick instance!");
        init_task_stack();
        systick.init_systic_interrupt_ms(KERNEL_TICK_PERIOD_MS, CORE_CLOCK_MHZ);

        update_to_next_task();
        switch_sp_to_psp();
        let entry = TASKS[CURRENT_TASK_IDX].task_handler;
        (entry)();
    }
}
