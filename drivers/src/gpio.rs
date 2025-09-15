
#![allow(dead_code)]


/// # GPIO Driver Module
///
/// This module provides low-level GPIO configuration and control functions for the STM32F407 microcontroller.
/// All register operations are performed through volatile reads and writes to memory-mapped hardware addresses.
///
/// ## Port and Pin Conventions
///
/// - `port: u32` — GPIO port index, where:
///   - `0` → GPIOA  
///   - `1` → GPIOB  
///   - ...  
///   - `8` → GPIOI
///
/// - `pin: u32` — GPIO pin number (valid range: `0` to `15`)
///
/// ## Parameter Value Conventions
///
/// ### `mode: u32` (used in `gpio_configure_mode`)
/// - `0` → Input  
/// - `1` → Output  
/// - `2` → Alternate Function  
/// - `3` → Analog
///
/// ### `output_type: u32` (used in `gpio_output_type_configure`)
/// - `0` → Push-Pull  
/// - `1` → Open-Drain
///
/// ### `output_speed: u32` (used in `gpio_output_speed_configure`)
/// - `0` → Low  
/// - `1` → Medium  
/// - `2` → Fast  
/// - `3` → High
///
/// ### `pull_up_down: u32` (used in `gpio_pulup_puldown_configure`)
/// - `0` → No Pull  
/// - `1` → Pull-Up  
/// - `2` → Pull-Down
///
/// ### `status: bool` (used in `gpio_write`)
/// - `true` → High  
/// - `false` → Low
///
/// ## Safety
///
/// While most functions in this module are `pub`, they internally perform `unsafe` register access.  
/// Users of this API must ensure that all passed values (e.g., `port`, `pin`) are within valid ranges and that memory-mapped registers are valid.  
/// All `unsafe` operations are confined to a small number of well-documented internal blocks.
//use crate::drivers::gpio_drive::gpio_macro::*;
use crate:: stm32f407_registers::*;
use crate::read_write::{read_register, write_register, reg_write_bit,reg_write_bits};
 


/// Function name: `select_gpio_base`  
///  
/// Description:  
/// Returns the base address of the GPIO peripheral corresponding to the given port number.  
///  
/// Safety:  
/// Safe function — no unsafe operations or memory access performed.  
///  
/// Parameters:  
/// - `port`: GPIO port index (0 for A, 1 for B, ..., 8 for I).  
///  
/// Return:  
/// - Base address (`u32`) of the selected GPIO port.
fn select_gpio_base(port: u32) -> u32 {
    match port {
        0 => GPIO_A_BASE,
        1 => GPIO_B_BASE,
        2 => GPIO_C_BASE,
        3 => GPIO_D_BASE,
        4 => GPIO_E_BASE,
        5 => GPIO_F_BASE,
        6 => GPIO_G_BASE,
        7 => GPIO_H_BASE,
        8 => GPIO_I_BASE,
        _ => panic!("Invalid GPIO port: {}. Valid range is 0 – 8.", port),
    }
}


/// Function name: `configure_gpio_clock`  
///  
/// Description:  
/// Enables or disables the AHB clock for a specified GPIO port.  
///  
/// Safety:  
/// Unsafe due to direct register access.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `enable`: `true` to enable, `false` to disable.  
///  
/// Return:  
/// - None
fn configure_gpio_clock(port: u32, enable: bool) {
    let rcc_ahbenr_addr = (RCC_BASE + 0x30) as *mut u32;

    unsafe {
        reg_write_bit(rcc_ahbenr_addr, port, enable);
    }
}

/// Function name: `gpio_configure_mode`  
///  
/// Description:  
/// Configures the mode (input, output, alternate function, analog) for a specific GPIO pin.  
///  
/// Safety:  
/// Unsafe due to raw pointer dereferencing and register manipulation.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `pin`: GPIO pin number (0–15).  
/// - `mode`: Mode value (0: Input, 1: Output, 2: Alt, 3: Analog).  
///  
/// Return:  
/// - None
pub fn gpio_configure_mode(port: u32, pin: u32, mode: u32) {
    configure_gpio_clock(port, true);
    let gpio_base = select_gpio_base(port);
    let moder_addr = gpio_base as *mut u32;

    unsafe {
        reg_write_bits(moder_addr, mode , pin * 2, 2);
        let actual_moder_value = read_register(moder_addr);
        write_register(moder_addr, actual_moder_value);
    }
}


/// Function name: `gpio_output_type_configure`  
///  
/// Description:  
/// Configures the output type of a GPIO pin as push-pull or open-drain.  
///  
/// Safety:  
/// Unsafe due to raw memory manipulation.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `pin`: GPIO pin number (0–15).  
/// - `output_type`: 0 for push-pull, 1 for open-drain.  
///  
/// Return:  
/// - None
pub fn gpio_output_type_configure(port: u32, pin: u32, output_type: u32) {
    assert!(pin < 16);
    assert!(output_type <= 1);

    let gpio_base = select_gpio_base(port);
    let otyper_addr = (gpio_base + 0x04) as *mut u32;

    unsafe {
        reg_write_bits(otyper_addr, output_type, pin, 1);
    }
}

/// Function name: `gpio_output_speed_configure`  
///  
/// Description:  
/// Configures the output speed of a GPIO pin.  
///  
/// Safety:  
/// Unsafe due to raw register access.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `pin`: GPIO pin number (0–15).  
/// - `output_speed`: Speed value (0: Low, 1: Medium, 2: Fast, 3: High).  
///  
/// Return:  
/// - None
pub fn gpio_output_speed_configure(port: u32, pin: u32, output_speed: u32) {
    assert!(pin < 16);
    assert!(output_speed <= 3);

    let gpio_base = select_gpio_base(port);
    let ospeedr_addr = (gpio_base + 0x08) as *mut u32;

    unsafe {
        reg_write_bits(ospeedr_addr, output_speed, pin * 2, 2);
    }
}

/// Function name: `gpio_pulup_puldown_configure`  
///  
/// Description:  
/// Configures pull-up/pull-down resistors for a GPIO pin.  
///  
/// Safety:  
/// Unsafe due to direct register access.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `pin`: GPIO pin number (0–15).  
/// - `pull_up_down`: 0: No pull, 1: Pull-up, 2: Pull-down.  
///  
/// Return:  
/// - None
pub fn gpio_pulup_puldown_configure(port: u32, pin: u32, pull_up_down: u32) {
    assert!(pin < 16);
    assert!(pull_up_down <= 2);

    let gpio_base = select_gpio_base(port);
    let pupdr_addr = (gpio_base + 0x0C) as *mut u32;

    unsafe {
        reg_write_bits(pupdr_addr, pull_up_down, pin * 2, 2);
    }
}

/// Function name: `gpio_read`  
///  
/// Description:  
/// Reads the logic level (high or low) from a GPIO pin.  
///  
/// Safety:  
/// Unsafe due to volatile memory access.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `pin`: GPIO pin number (0–15).  
///  
/// Return:  
/// - `bool`: `true` if high, `false` if low.
pub fn gpio_read(port: u32, pin: u32) -> bool {
    assert!(pin < 16);

    let gpio_base = select_gpio_base(port);
    let idr_addr = (gpio_base + 0x10) as *const u32;

    unsafe {
        let value = read_register(idr_addr as *mut u32);
        (value & (1 << pin)) != 0
    }
}

/// Function name: `gpio_write`  
///  
/// Description:  
/// Sets the output state of a GPIO pin to high or low.  
///  
/// Safety:  
/// Unsafe due to direct memory writes.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `pin`: GPIO pin number (0–15).  
/// - `status`: `true` for high, `false` for low.  
///  
/// Return:  
/// - None
pub fn gpio_write(port: u32, pin: u32, status: bool) {
    assert!(pin < 16);

    let gpio_base = select_gpio_base(port);
    let odr_addr = (gpio_base + 0x14) as *mut u32;

    unsafe {
        reg_write_bit(odr_addr, pin, status);
    }
}

/// Function name: `toggle_gpio`  
///  
/// Description:  
/// Toggles the current output state of a GPIO pin.  
///  
/// Safety:  
/// Unsafe due to raw register reads/writes.  
///  
/// Parameters:  
/// - `port`: GPIO port number.  
/// - `pin`: GPIO pin number (0–15).  
///  
/// Return:  
/// - None
pub fn toggle_gpio(port: u32, pin: u32) {
    assert!(pin < 16);

    let gpio_base = select_gpio_base(port);
    let odr_addr = (gpio_base + 0x14) as *mut u32;

    unsafe {
        let value = read_register(odr_addr);
        let toggled = value ^ (1 << pin);
        write_register(odr_addr, toggled);
    }
}
