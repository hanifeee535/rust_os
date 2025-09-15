#![allow(clippy::empty_loop)]

use core::ptr::{read_volatile, write_volatile};
use cortex_m_rt::{exception};
use crate::os_config::*;
use crate::systick::{SysTick};
use cortex_m::interrupt;

pub const CORE_CLOCK_MHZ: u32 = 16; // HSI default; adjust to 168 if PLL used

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

#[unsafe(no_mangle)]
pub extern "C" fn update_to_next_task() {
    unsafe {
        for _ in 0..MAX_TASK {
            // Move to next task index
            CURRENT_TASK_IDX = (CURRENT_TASK_IDX + 1) % MAX_TASK;

            // Check if READY
            if TASKS[CURRENT_TASK_IDX].current_state == TASK_READY_STATE {
                break;
            }
        }
    }
}

/// Trigger a PendSV to request a context switch.
pub fn schedule() {
    unsafe {       
        let val = read_volatile(SCB_ICSR);
        write_volatile(SCB_ICSR, val | (1 << 28)); // Set PENDSVSET bit
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

        // Set PendSV to lowest priority (0xFF)
        let shpr2 = 0xE000_ED20 as *mut u32;
        let val = read_volatile(shpr2);
        write_volatile(shpr2, (val & !(0xFFu32 << 24)) | (0xFFu32 << 24));

        // Set SysTick to high priority (0x00)
        let shpr3 = 0xE000_ED24 as *mut u32;
        let val = read_volatile(shpr3);
        write_volatile(shpr3, (val & !(0xFFu32 << 24)) | (0x00u32 << 24));

        // Disable lazy FPU stacking
        let fpccr = 0xE000_EF34 as *mut u32;
        let val = read_volatile(fpccr);
        write_volatile(fpccr, (val | (1 << 31)) & !(1 << 30));

        let mut systick = SysTick::take().expect("Failed to take SysTick instance!");
        init_task_stack();
        systick.init_systic_interrupt_ms(KERNEL_TICK_PERIOD_MS, CORE_CLOCK_MHZ);

        update_to_next_task(); // Sets CURRENT_TASK_IDX to 1 (task 1)
        switch_sp_to_psp();
        let entry = TASKS[CURRENT_TASK_IDX].task_handler;
        (entry)();
    }
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     loop {} // Debug: inspect ef in debugger
// }