#![allow(dead_code)]

/// # Read/Write Utilities for Memory-Mapped Registers
///
/// This module provides low-level, volatile-safe functions for interacting with 32-bit memory-mapped hardware registers.
/// These functions are specifically tailored for programming on STM32 (and similar) microcontrollers,
/// where direct register manipulation is required.
///
/// ## Pointer and Value Types
///
/// - `register_address: *mut u32` — Raw mutable pointer to a 32-bit hardware register.
/// - `value: u32` — 32-bit value used for writing or comparison.
/// - `bit_position: u32` — Bit index within a 32-bit register (valid range: `0` to `31`).
/// - `bit_value: bool` — Boolean value:
///   - `true` → Set bit (1)
///   - `false` → Clear bit (0)
/// - `start_bit: u32` — Start index of a bit field (0-based, must be less than 32).
/// - `num_bits: u32` — Number of bits in the field (must be between 1 and 32, inclusive).
///
/// ## Function Behavior
///
/// - `read_register(register_address: *mut u32) -> u32`  
///   Performs a volatile read from a 32-bit register.
///
/// - `write_register(register_address: *mut u32, value: u32)`  
///   Performs a volatile write to a 32-bit register.
///
/// - `reg_write_value(register_address: *mut u32, new_register_value: u32)`  
///   Overwrites the entire 32-bit register with a new value.
///
/// - `reg_write_bit(register_address: *mut u32, bit_position: u32, bit_value: bool)`  
///   Sets or clears an individual bit while preserving other bits.
///
/// - `reg_write_bits(register_address: *mut u32, value_to_set: u32, start_bit: u32, num_bits: u32)`  
///   Writes a range of bits (bit field) to the register without affecting other bits.
///
/// - `reg_read_bit_value(register_address: *mut u32, bit_position: u32) -> bool`  
///   Reads a specific bit from the register and returns it as a boolean.
///
/// ## Safety
///
/// All functions in this module are marked `unsafe` because they dereference raw pointers and directly access
/// memory-mapped hardware. Callers **must** guarantee:
///
/// - `register_address` is valid, correctly aligned, and points to a readable or writable memory-mapped register.
/// - Bit indices (`bit_position`, `start_bit`, `num_bits`) are within valid bounds.
/// - Concurrency rules are followed to prevent data races or unintended hardware interactions.
///
/// These functions are designed to encapsulate all `unsafe` logic in one place, allowing higher-level abstractions
/// to be written with `safe` interfaces.
use core::ptr;

/// Function name: read_register  
///  
/// Description:  
/// Reads a 32-bit value from the given memory-mapped hardware register address.  
///  
/// # Safety  
/// Caller must ensure that the `add` pointer:  
/// - Is non-null and valid for reads.  
/// - Is properly aligned for `u32`.  
/// - Points to a readable memory-mapped hardware register.  
///  
/// # Parameters  
/// - `add`: A mutable pointer to a 32-bit hardware register address.  
///  
/// # Return  
/// - The 32-bit value currently stored at the register address.
pub unsafe fn read_register(add: *mut u32) -> u32 {
    unsafe {
        ptr::read_volatile(add)
    }
}

/// Function name: write_register  
///  
/// Description:  
/// Writes a 32-bit value to the given memory-mapped hardware register address.  
///  
/// # Safety  
/// Caller must ensure that the `add` pointer:  
/// - Is non-null and valid for writes.  
/// - Is properly aligned for `u32`.  
/// - Points to a writable memory-mapped hardware register.  
///  
/// # Parameters  
/// - `add`: A mutable pointer to a 32-bit hardware register address.  
/// - `value`: The 32-bit value to write to the register.  
///  
/// # Return  
/// - None
pub unsafe fn write_register(add: *mut u32, value: u32) {
    unsafe {
        ptr::write_volatile(add, value)
    }
}

/// Function name: reg_write_bits  
///  
/// Description:  
/// Writes a specific bit field in a 32-bit register without affecting other bits.  
/// Clears the target field, masks and shifts the input value, and writes it into position.  
///  
/// # Safety  
/// Caller must ensure that `register_address` is:  
/// - A valid, aligned pointer to a 32-bit hardware register.  
/// - Not concurrently accessed from another context unless synchronized.  
///  
/// # Parameters  
/// - `register_address`: Pointer to the hardware register.  
/// - `value_to_set`: Value to write into the bit field (only the relevant bits will be used).  
/// - `start_bit`: Starting bit position (must be < 32).  
/// - `num_bits`: Number of bits to write (must be > 0 and ≤ 32, must not overflow beyond bit 31).  
///  
/// # Return  
/// - None
pub unsafe fn reg_write_bits(register_address: *mut u32, value_to_set: u32, start_bit: u32, num_bits: u32) {
    assert!(
        num_bits > 0 && num_bits <= 32,
        "num_bits must be between 1 and 32"
    );
    assert!(
        start_bit < 32,
        "start_bit must be less than 32 (0-indexed for a 32-bit register)"
    );
    assert!(
        (start_bit + num_bits) <= 32,
        "Bit field extends beyond the 32-bit register boundary"
    );

    unsafe {
        let current_register_value = read_register(register_address);

        let bits_of_ones_mask = (1 << num_bits) - 1;
        let clear_mask = bits_of_ones_mask << start_bit;

        let shifted_value_to_set = (value_to_set << start_bit) & clear_mask;

        let new_register_value = (current_register_value & !clear_mask) | shifted_value_to_set;

        write_register(register_address, new_register_value);
    }
}

/// Function name: reg_write_bit  
///  
/// Description:  
/// Sets or clears a specific bit in a 32-bit register while preserving all other bits.  
///  
/// # Safety  
/// Caller must ensure that `register_address`:  
/// - Is a valid, aligned pointer to a 32-bit hardware register.  
/// - Is safe to write to in the current context.  
///  
/// # Parameters  
/// - `register_address`: Pointer to the hardware register.  
/// - `bit_position`: Bit to modify (must be between 0 and 31).  
/// - `bit_value`: Boolean value to write — `true` sets the bit, `false` clears it.  
///  
/// # Return  
/// - None
pub unsafe fn reg_write_bit(register_address: *mut u32, bit_position: u32, bit_value: bool) {
    assert!(
        bit_position < 32,
        "bit_position must be less than 32 (0-indexed for a 32-bit register)"
    );

    unsafe {
        let current_register_value = read_register(register_address);
        let bit_mask = 1 << bit_position;

        let new_register_value = if bit_value {
            current_register_value | bit_mask
        } else {
            current_register_value & !bit_mask
        };

        write_register(register_address, new_register_value);
    }
}

/// Function name: reg_write_value  
///  
/// Description:  
/// Overwrites the entire 32-bit register with a new value.  
///  
/// # Safety  
/// Caller must ensure that `register_address`:  
/// - Points to a valid, aligned 32-bit hardware register.  
/// - Is safe to write to in the current hardware context.  
///  
/// # Parameters  
/// - `register_address`: Pointer to the hardware register.  
/// - `new_register_value`: The full 32-bit value to be written.  
///  
/// # Return  
/// - None
pub unsafe fn reg_write_value(register_address: *mut u32, new_register_value: u32) {
    unsafe {
        write_register(register_address, new_register_value);
    }
}


/// Function name: reg_read_bit_value  
///  
/// Description:  
/// Reads and returns the boolean value of a specific bit in a 32-bit register.  
///  
/// # Safety  
/// Caller must ensure that `register_address`:  
/// - Points to a valid, aligned 32-bit hardware register.  
/// - Is readable and not being concurrently modified without synchronization.  
///  
/// # Parameters  
/// - `register_address`: Pointer to the hardware register.  
/// - `bit_position`: Bit to read (must be between 0 and 31).  
///  
/// # Return  
/// - `true` if the specified bit is set (1), `false` if it is clear (0).
pub unsafe fn reg_read_bit_value(register_address: *mut u32, bit_position: u32) -> bool {
    assert!(
        bit_position < 32,
        "bit_position must be less than 32 (0-indexed for a 32-bit register)"
    );

    unsafe {
        let current_register_value = read_register(register_address);
        let bit_mask = 1 << bit_position;
        (current_register_value & bit_mask) != 0
    }
}
