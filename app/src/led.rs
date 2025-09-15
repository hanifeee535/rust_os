
#![allow(dead_code)]
use drivers::gpio::*;

pub const PORTA : u32 = 0;
pub const PORTB : u32 = 1;
pub const PORTC : u32 = 2;
pub const PORTD : u32 = 3;

pub const GPIO_MODE_INPUT: u32 = 0;
pub const GPIO_MODE_GP_OUTPUT: u32 = 1;
pub const GPIO_MODE_ALTERNATE: u32 = 2;
pub const GPIO_MODE_ANALOG: u32 = 3;

pub const GPIO_OUTPUT_PUSH_PULL : u32 = 0;
pub const GPIO_OUTPUT_OPEN_DRAIN : u32 = 1;

pub const LED_OFF : bool = false;
pub const LED_ON : bool = true;
pub const LED_PORT : u32 = PORTD;
pub const LED_1_PIN : u32 = 12;
pub const LED_2_PIN : u32 = 13;
pub const LED_3_PIN : u32 = 14;
pub const LED_4_PIN : u32 = 15;




pub fn init_led(){
    gpio_configure_mode (LED_PORT, LED_1_PIN, GPIO_MODE_GP_OUTPUT);
    gpio_configure_mode (LED_PORT, LED_2_PIN, GPIO_MODE_GP_OUTPUT);
    gpio_configure_mode (LED_PORT, LED_3_PIN, GPIO_MODE_GP_OUTPUT);
    gpio_configure_mode (LED_PORT, LED_4_PIN, GPIO_MODE_GP_OUTPUT);

    gpio_output_type_configure (LED_PORT, LED_1_PIN, GPIO_OUTPUT_PUSH_PULL );
    gpio_output_type_configure (LED_PORT, LED_2_PIN, GPIO_OUTPUT_PUSH_PULL );
    gpio_output_type_configure (LED_PORT, LED_3_PIN, GPIO_OUTPUT_PUSH_PULL );
    gpio_output_type_configure (LED_PORT, LED_4_PIN, GPIO_OUTPUT_PUSH_PULL );
}


pub fn led1_on(){
    gpio_write (LED_PORT, LED_1_PIN,LED_ON);   

}

pub fn led2_on(){
    gpio_write (LED_PORT, LED_2_PIN,LED_ON);   

}
pub fn led3_on(){
    gpio_write (LED_PORT, LED_3_PIN,LED_ON);   

}
pub fn led4_on(){
    gpio_write (LED_PORT, LED_4_PIN,LED_ON);   

}

pub fn led1_toggle(){
    toggle_gpio(LED_PORT, LED_1_PIN);  

}

pub fn led2_toggle(){
   toggle_gpio(LED_PORT, LED_2_PIN);  

}
pub fn led3_toggle(){
    toggle_gpio(LED_PORT, LED_3_PIN);    

}
pub fn led4_toggle(){
    toggle_gpio(LED_PORT, LED_4_PIN);    

}