#![allow(dead_code)]

/// # EXTI (External Interrupt) Configuration Module
///
/// This module provides functions to configure external interrupts (EXTI) on the STM32F407 MCU.
///
/// ## Common Parameters
///
/// - `port: u32`  
///   GPIO port number, where:  
///   `0` → GPIOA, `1` → GPIOB, ..., `8` → GPIOI.
///
/// - `pin: u32`  
///   GPIO pin number or EXTI line number (valid range: `0` to `15`).
///
/// - `status: bool`  
///   Boolean flag indicating enable (`true`) or disable (`false`).
///
/// - `trigger_type: u32`  
///   Interrupt trigger type:  
///   `0` → Rising edge trigger  
///   `1` → Falling edge trigger
///
use crate:: stm32f407_registers::*;
use crate::read_write::{read_register, write_register};
use crate::cortex_m4::enable_irq;



/// Function name: configure_syscfgen_clock  
///  
/// Description:  
/// Enables or disables the system configuration controller clock by setting or clearing  
/// the RCC APB2ENR register bit 14 (SYSCFG clock enable).  
///  
/// # Parameters  
/// - `status`: Boolean flag to enable (`true`) or disable (`false`) the SYSCFG clock.  
///  
/// # Return  
/// - None
fn configure_syscfgen_clock (status: bool){
    let rcc_apb2enr_addr = (RCC_BASE + 0x44) as *mut u32;
    unsafe {
        let apb2enr_value : u32 = read_register(rcc_apb2enr_addr);
        let  updated_apb2_value: u32 = if status {
              apb2enr_value | (1<<14)
        }
        else {
             apb2enr_value & !(1<<14)
        };
        
        write_register(rcc_apb2enr_addr, updated_apb2_value);
    }
}

/// Function name: map_exti_line  
///  
/// Description:  
/// Maps a GPIO port and pin to the corresponding EXTI line by updating the correct SYSCFG_EXTICR register.  
/// Each pin maps to a 4-bit field inside one of the four EXTICR registers.  
///  
/// # Parameters  
/// - `port`: GPIO port number (0 for GPIOA, 1 for GPIOB, etc.)  
/// - `pin`: GPIO pin number (0–15)  
///  
/// # Panics  
/// Panics if `pin` is greater than 15.  
///  
/// # Return  
/// - None
fn map_exti_line(port: u32, pin: u32){

     let syscfg_exticr: u32 = if pin <= 3 {
        SYSCFG_BASE + 0x08
    } else if pin <= 7 {
        SYSCFG_BASE + 0x0C
    } else if pin <= 11 {
        SYSCFG_BASE + 0x10
    } else if pin <= 15 {
        SYSCFG_BASE + 0x14
    } else {
        panic!("Invalid pin number: {}", pin);
    };

    let syscfg_exticr_address: *mut u32 = syscfg_exticr as *mut u32;
    unsafe {
        let mut reg_value = read_register(syscfg_exticr_address);
        let shift = (pin % 4) * 4;
        let mask = 0b1111 << shift;

        // Clear the current bits at that position and set the new port value
        reg_value &= !mask;
        reg_value |= (port & 0b1111) << shift;

        write_register(syscfg_exticr_address, reg_value);
        
    }

}


/// Function name: config_interrupt_trigger  
///  
/// Description:  
/// Configures the interrupt trigger type (rising or falling edge) for a specific EXTI pin by  
/// modifying the EXTI_RTSR or EXTI_FTSR registers.  
///  
/// # Parameters  
/// - `pin`: EXTI line number (0–15)  
/// - `trigger_type`: Trigger type (`0` = rising edge, `1` = falling edge)  
///  
/// # Panics  
/// Panics if `pin` is greater than 15 or if `trigger_type` is invalid.  
///  
/// # Return  
/// - None
fn config_interrupt_trigger(pin: u32, trigger_type: u32){
    
    if pin > 15 {
        panic!("Invalid EXTI pin: {}", pin);
    }
    let trigger_resister = if trigger_type == 0{
        EXTI_BASE + 0x08 //rising trigger
    } else if trigger_type == 1{
        EXTI_BASE + 0x0C  // EXTI_FTSR (falling trigger)
    } else {
         panic!("Invalid trigger type: {}", trigger_type);
    };

    let trigger_resister_address = trigger_resister as *mut u32;

    unsafe {
        let mut reg_value = read_register(trigger_resister_address);
        let bit = 1<<pin;
        if trigger_type == 0{ //rising
            reg_value |= bit; 
        }
        else if  trigger_type == 1 { //falling
            reg_value &= !bit;
        }
        else {
           
         panic!("Invalid trigger type: {}", trigger_type);
        }
        write_register(trigger_resister_address, reg_value);
    }

    

}


/// Function name: configure_interrupt_mask_register  
///  
/// Description:  
/// Enables or disables the interrupt mask for the specified EXTI pin by updating the EXTI_IMR register.  
///  
/// # Parameters  
/// - `pin`: EXTI line number (0–15)  
/// - `status`: `true` to unmask (enable), `false` to mask (disable) the interrupt  
///  
/// # Panics  
/// Panics if `pin` is greater than 15.  
///  
/// # Return  
/// - None
pub fn configure_interrupt_mask_register(pin: u32, status: bool) {
    let exti_imr_addr = EXTI_BASE as *mut u32;
    
    unsafe {
        let mut imr_value = read_register(exti_imr_addr);

        if pin > 15 {
            panic!("Invalid EXTI pin: {}", pin);  
        }

        if status {
            // Unmask interrupt (enable)
            imr_value |= 1 << pin;
        } else {
            // Mask interrupt (disable)
            imr_value &= !(1 << pin);
        }

        write_register(exti_imr_addr, imr_value);
    }
}


/// Enables the NVIC interrupt for a given EXTI pin by mapping the pin
/// to the corresponding IRQ number and enabling it.
///
/// # Parameters
/// - `pin`: EXTI line number (0–15)
///
/// # Panics
/// Panics if the `pin` is not in the valid range.
fn enable_nvic_interrupt(pin: u32) {
    let irq_number = match pin {
        0 => 6,          // EXTI0
        1 => 7,          // EXTI1
        2 => 8,          // EXTI2
        3 => 9,          // EXTI3
        4 => 10,         // EXTI4
        5..=9 => 23,     // EXTI9_5
        10..=15 => 40,   // EXTI15_10
        _ => panic!("Invalid EXTI pin: {}", pin),
    };

    enable_irq(irq_number);
}



/// Function name: configure_gpio_interrupt  
///  
/// Description:  
/// Performs full configuration of a GPIO external interrupt including enabling SYSCFG clock,  
/// mapping EXTI line to port/pin, enabling interrupt mask, setting trigger type, and enabling NVIC interrupt.  
///  
/// # Parameters  
/// - `port`: GPIO port number (0 for GPIOA, etc.)  
/// - `pin`: GPIO pin number (0–15)  
/// - `trigger_type`: Trigger type (`0` = rising edge, `1` = falling edge)  
///  
/// # Return  
/// - None
pub fn configure_gpio_interrupt (port: u32, pin: u32, trigger_type: u32){
    //1. configure system configuration controller clock 
    configure_syscfgen_clock (true);
    //2. enable external interrupt configuration register
    map_exti_line(port, pin);
    //3. Enabling interrupt mask register
    configure_interrupt_mask_register(pin, true);
    //4. select interrupt trigger type
    config_interrupt_trigger(pin, trigger_type);
    //5. NVIC enable IRQ
    enable_nvic_interrupt(pin);
}




/// Function name: clear_exti_pending  
///  
/// Description:  
/// Clears the pending interrupt flag for the specified EXTI line by writing a `1` to the EXTI_PR register.  
///  
/// # Parameters  
/// - `pin`: EXTI line number (0–15)  
///  
/// # Panics  
/// Panics if `pin` is greater than 15.  
///  
/// # Return  
/// - None
pub fn clear_exti_pending(pin: u32) {
    if pin > 15 {
        panic!("Invalid EXTI pin: {}", pin);
    }

    let exti_pr_addr = (EXTI_BASE + 0x14) as *mut u32; // EXTI_PR

    unsafe {
        write_register(exti_pr_addr, 1 << pin);
    }
}
