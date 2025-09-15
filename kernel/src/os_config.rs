// //! # Scheduler Configuration for Cortex-M4 Microcontrollers
// //!
// //! This module provides all configuration constants used by the scheduler,
// //! designed to work on **any Cortex-M4 microcontroller without the FPU** (e.g., STM32F4, NXP, TI Tiva).
// //!
// //! It defines stack sizes, memory regions, task limits, and system tick frequency.
// //!
// //! ##  Adapting to targeted Microcontroller
// //!
// //! To configure this scheduler work with the target MCU:

// //! - Adjust MAX_TASK, SIZE_TASK_STACK, SRAM_* to match targeted MCU.
// //! - Provide the task handler functions in app crate (see example later).
// //! - Pick the first task to act as the idle task.
// //! - Check **SRAM size and starting address**
// //! - Refer to the **memory map** in device’s reference manual or datasheet
// //! - Set the correct values for `SRAM_START` and `SRAM_SIZE`
// //! - Make sure the combined stack size (`MAX_TASK × SIZE_TASK_STACK + SIZE_SCHEDULER_STACK`) **fits within SRAM**
// //!
// //! ###  Example for STM32F407 (Cortex-M4)
// //! - `SRAM_START`: `0x2000_0000`
// //! - `SRAM_SIZE`:  `128 * 1024` (128KB total SRAM)
// //!
// //! ---


// Kernel tick period in milliseconds.
// Lower => more frequent switching. Higher => less frequent.
pub const KERNEL_TICK_PERIOD_MS: u32 = 1;

// Maximum number of concurrent tasks (must match array sizes in scheduler).
// Keep this modest for small MCUs. If maximum number of task needs to be changed, update stack and other arrays accordingly.
pub const MAX_TASK: usize = 4;

// Size of each task's private stack in bytes (must be multiple of 8).
pub const SIZE_TASK_STACK: u32 = 1024; // 2 KB

// Size of scheduler (MSP) stack in bytes
pub const SIZE_SCHEDULER_STACK: u32 = 1024; // 1 KB

// SRAM base and size — set these according to the MCU memory map
pub const SRAM_START: u32 = 0x2000_0000;
pub const SRAM_SIZE: u32 = 128 * 1024; // 128 KB
pub const SRAM_END: u32 = SRAM_START + SRAM_SIZE;

// // Total stack usage for compile-time check
// pub const TOTAL_STACK_USAGE: u32 = (MAX_TASK as u32 * SIZE_TASK_STACK) + SIZE_SCHEDULER_STACK;

// // /// Compile-time check: ensure all stacks fit into SRAM.
// // /// If TOTAL_STACK_USAGE > SRAM_SIZE, the subtraction underflows and this fails to compile.
// // const _: [u8; (SRAM_SIZE - TOTAL_STACK_USAGE) as usize + 1] = [0u8; 1];

/// Compute top-of-stack for task `i` (0..MAX_TASK-1). Full descending stack.
#[inline(always)]
pub const fn task_stack_start(i: usize) -> u32 {
    SRAM_END - (i as u32 * SIZE_TASK_STACK)
}

/// Scheduler stack start (MSP) below task stacks
#[inline(always)]
pub const fn scheduler_stack_start() -> u32 {
    SRAM_END - (MAX_TASK as u32 * SIZE_TASK_STACK)
}

/// Task states
pub const TASK_READY_STATE: u8 = 0x00;
pub const TASK_BLOCKED_STATE: u8 = 0xFF;

/// Default xPSR value for initial stack frame (Thumb bit set)
pub const DUMMY_XPSR: u32 = 0x0100_0000;

/// Task handler ABI: use C ABI because scheduler enters tasks from assembly
pub type TaskHandler = unsafe extern "C" fn();

/// Task Control Block (TCB).
/// We derive `Clone` only because we'll initialize the array at runtime.
#[repr(C)]
#[derive(Copy,Clone)]
pub struct Tcb {
    pub psp_value: u32,     // Process Stack Pointer for the task
    pub priority: u8,       // Higher number => higher priority
    pub current_state: u8,  // TASK_READY_STATE or TASK_BLOCKED_STATE
    pub block_count: u8,    // blocking counter (if used)
    pub task_handler: TaskHandler,
}

// ---------- Extern declarations for task handlers ----------
// Implement these functions in app (main.rs). Example:
//   #[unsafe(no_mangle)] pub extern "C" fn task1_handler() { loop { /* work */ } }
//
// These are `extern "C"` (FFI) functions; calling them is unsafe.
unsafe extern "C" {
    pub unsafe fn Idle_task_handler();
    pub unsafe fn task1_handler();
    pub unsafe fn task2_handler();
    pub unsafe fn task3_handler();

}

/// Static array of all TCBS for tasks.
/// Initialize stacks and other fields at runtime during scheduler init.
pub static mut TASKS: [Tcb; MAX_TASK] = [
    Tcb { psp_value: 0, priority: 0, current_state: TASK_READY_STATE, block_count: 0, task_handler: Idle_task_handler },
    Tcb { psp_value: 0, priority: 3, current_state: TASK_READY_STATE, block_count: 0, task_handler: task1_handler },
    Tcb { psp_value: 0, priority: 3, current_state: TASK_READY_STATE, block_count: 0, task_handler: task2_handler },
    Tcb { psp_value: 0, priority: 4, current_state: TASK_READY_STATE, block_count: 0, task_handler: task3_handler },
   ];