#![allow(dead_code)]


use crate:: stm32f407_registers::*;
use crate::read_write::{read_register, write_register};


/// Enables the IRQ for the given IRQ number by setting the appropriate
/// bit in the NVIC ISER register.
///
/// # Parameters
/// - `irq_number`: The IRQ number to enable.
///
/// # Safety
/// Assumes `irq_number` is valid and within NVIC supported IRQ range.
pub fn enable_irq(irq_number: u32) {
    let register_offset = (irq_number / 32) * 4;
    let bit_position = irq_number % 32;
    let iser_addr = (NVIC_ISER + register_offset) as *mut u32;
    unsafe {
        let mut value = read_register(iser_addr);
        value |= 1 << bit_position;
        write_register(iser_addr, value);
    }
}


/// Disables the IRQ for the given IRQ number by setting the appropriate
/// bit in the NVIC ICER register.
///
/// # Parameters
/// - `irq_number`: The IRQ number to disable.
///
/// # Safety
/// Assumes `irq_number` is valid and within NVIC supported IRQ range.
pub fn disable_irq(irq_number: u32) {
    let register_offset = (irq_number / 32) * 4;
    let bit_position = irq_number % 32;
    let icer_addr = (NVIC_ICER + register_offset) as *mut u32;
    unsafe {
        let mut value = read_register(icer_addr);
        value |= 1 << bit_position;
        write_register(icer_addr, value);
    }
}


/// Function name: disable_global_interrupt
///
/// Description:
/// Disables all maskable interrupts globally by setting the PRIMASK register.
/// This prevents any IRQ (Interrupt Requests) from interrupting the processor,
/// except for non-maskable interrupts (NMI) and hard fault exceptions.
/// This is useful when entering a critical section of code where atomicity is required.
///
/// # Safety
/// - Disabling interrupts globally can lead to system deadlocks or missed critical events if
///   used improperly.
/// - The caller must ensure the system can safely tolerate interrupts being disabled.
/// - This should be used sparingly and interrupts should be re-enabled as soon as possible.
///
/// # Parameters
/// - None
///
/// # Return
/// - None
pub fn disable_global_interrupt() {
    unsafe {
        core::arch::asm!("cpsid i", options(nomem, nostack, preserves_flags));
    }
}

/// Function name: enable_global_interrupt
///
/// Description:
/// Enables all maskable interrupts globally by clearing the PRIMASK register.
/// This allows IRQ interrupts to be processed by the CPU again after they have
/// been disabled. Non-maskable interrupts (NMI) and hard faults are always enabled.
///
/// # Safety
/// - The caller must ensure the system is in a consistent state to handle interrupts
///   before enabling them.
/// - Enabling interrupts prematurely (e.g., before system initialization) can cause
///   unexpected behavior.
///
/// # Parameters
/// - None
///
/// # Return
/// - None
pub fn enable_global_interrupt() {
    unsafe {
        core::arch::asm!("cpsie i", options(nomem, nostack, preserves_flags));
    }
}



/// Function name: set_interrupt_priority
///
/// Description:
/// Sets the priority level of a specific IRQ number in the NVIC (Nested Vectored Interrupt Controller).
/// Lower numerical values correspond to higher priority (0 = highest priority).
/// STM32F407 supports 4 bits of priority (0..15) by default, but actual implemented bits may vary.
///
/// # Safety
/// - Caller must ensure `irq_number` is valid and corresponds to an IRQ supported by the MCU.
/// - `priority` must be within the valid priority range supported by the device (usually 0..15).
///
/// # Parameters
/// - `irq_number`: The IRQ number to set priority for.
/// - `priority`: The priority value to assign (lower is higher priority).
///
/// # Return
/// - None
pub fn set_interrupt_priority(irq_number: u32, priority: u8) {
    
    if irq_number >= 240 {
        panic!("Invalid IRQ number");
    }
    let ipr_addr = (NVIC_IPR + irq_number) as *mut u8;

    unsafe {
        // Priority registers are 8-bit wide; STM32F407 uses upper 4 bits for priority
        let priority_value = priority << 4;
        write_register(ipr_addr as *mut u32, priority_value as u32);
    }
}

/// Function name: set_interrupt_priority_grouping
///
/// Description:
/// Configures the priority grouping of the NVIC, determining the split between
/// preemption priority and subpriority (used for nested interrupt behavior).
/// This is done by writing to the AIRCR (Application Interrupt and Reset Control Register)
/// in the System Control Block (SCB).
///
/// The grouping defines how many bits are used for preemption priority and
/// how many bits for subpriority.
///
/// # Safety
/// - This function should be called with interrupts disabled to avoid race conditions.
/// - The `priority_group` must be within the range 0..=7.
///
/// # Parameters
/// - `priority_group`: The priority grouping value (0..7).
///
/// # Return
/// - None
pub fn set_interrupt_priority_grouping(priority_group: u8) {
    const VECTKEY_MASK: u32 = 0xFFFF_0000;
    const VECTKEY: u32 = 0x5FA << 16;
    const PRIGROUP_MASK: u32 = 0x700; // Bits 10:8 for PRIGROUP field

    if priority_group > 7 {
        panic!("Invalid priority group");
    }

    let scb_aircr = SCB_AIRCR_BASE as *mut u32;

    unsafe {
        let current = read_register(scb_aircr);
        let new_value = (current & !PRIGROUP_MASK) | VECTKEY | ((priority_group as u32) << 8);
        write_register(scb_aircr, new_value);
    }
}