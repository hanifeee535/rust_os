
//declaring the registers

//RCC register
pub const RCC_BASE: u32 =    0x4002_3800;


//system config register
pub const SYSCFG_BASE: u32 =  0x4001_3800;

//GPIO Registers
pub const GPIO_A_BASE: u32 = 0x4002_0000;
pub const GPIO_B_BASE: u32 = 0x4002_0400;
pub const GPIO_C_BASE: u32 = 0x4002_0800;
pub const GPIO_D_BASE: u32 = 0x4002_0C00;
pub const GPIO_E_BASE: u32 = 0x4002_1000;
pub const GPIO_F_BASE: u32 = 0x4002_1400;
pub const GPIO_G_BASE: u32 = 0x4002_1800;
pub const GPIO_H_BASE: u32 = 0x4002_1C00;
pub const GPIO_I_BASE: u32 = 0x4002_2000;


//exti register
pub const EXTI_BASE : u32 = 0x4001_3C00;

//NVIC:
pub const NVIC_BASE : u32 = 0xE000_E100;
pub const NVIC_ISER: u32 = NVIC_BASE;
pub const NVIC_ICER: u32 = NVIC_BASE+ 0x80;
pub const NVIC_IPR: u32 = 0xE000_E400;


//SCB
pub const SCB_AIRCR_BASE: u32 = 0xE000_ED0C;

//Systic
pub const SYSTICK_BASE : u32 = 0xE000_E010;


