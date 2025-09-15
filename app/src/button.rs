

#![allow(dead_code)]
use drivers::gpio::*;
use crate:: led::*;

pub const GPIO_MODE_INPUT: u32 = 0;

pub const PORTA: u32 = 0;
pub const BUTTON_PIN :u32 = 0;
pub const BUTTON_PORT : u32 = PORTA;

pub fn init_user_button(){
    gpio_configure_mode (BUTTON_PORT, BUTTON_PIN, GPIO_MODE_INPUT);

}

// pub fn led_control_with_button() {
//     let button_status = gpio_read(BUTTON_PORT, BUTTON_PIN);

//     if button_status {
//         led_on();
//     } else {
//         led_off();
//     }

   
// }
