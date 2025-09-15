
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]


mod led;
mod button;
use cortex_m_rt:: {entry};


//use drivers::exti::*;
//use button::*;
//use drivers::systick::{SysTick};
use kernel::os::*;
use kernel::os_config::*;
use crate:: led::*;
//use drivers::gpio::*; 
use core::panic::PanicInfo;


// #[allow(non_snake_case)]
// #[unsafe(no_mangle)] 
// fn EXTI0_Handler(){
//     clear_exti_pending(0);
//     toggle_led();
// }


// #[exception]
// fn SysTick() {    
//     toggle_led();
// }

//const CORE_CLOCK_MHZ: u32 = 8;


#[entry]
fn main() -> ! {
   
    // let mut systick = SysTick::take().expect("Failed to take SysTick instance! It's likely already in use.");
    //systick.init(7999, ClockSource::Core);   

    init_led();
    
    scheduler_init();
    

    loop {



    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn Idle_task_handler() {
    loop {
        led1_toggle();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn task1_handler() {
    loop {
       led2_toggle();
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn task2_handler() {
    loop {
        led3_toggle();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn task3_handler() {
    loop {
        led4_toggle();
    }
}
