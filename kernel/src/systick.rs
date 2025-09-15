#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

pub const SYSTICK_BASE : u32 = 0xE000_E010;

#[repr(C)]
struct SysTickRegisters {
    st_csr: u32,
    st_rvr: u32,
    st_cvr: u32,
    calib: u32,
}

pub enum ClockSource {
    External,
    Core,
}

pub struct SysTick {
    _private: (),
}

static mut TAKEN: bool = false;

const SYSTICK_RVR_MAX: u32 = 0x00FF_FFFF;
const SYSTICK_CSR_ENABLE_BIT: u32 = 0;
const SYSTICK_CSR_TICKINT_BIT: u32 = 1;
const SYSTICK_CSR_CLKSOURCE_BIT: u32 = 2;
const SYSTICK_CSR_COUNTFLAG_BIT: u32 = 16;

impl SysTick {
    pub fn take() -> Option<Self> {
        unsafe {
            if TAKEN {
                None
            } else {
                TAKEN = true;
                Some(SysTick { _private: () })
            }
        }
    }

    #[inline(always)]
    fn regs() -> *mut SysTickRegisters {
        SYSTICK_BASE as *mut SysTickRegisters
    }

    /// Init SysTick (no interrupt setup here)
    pub fn init(&mut self, reload: u32, clk_source: ClockSource) {
        unsafe {
            let regs = Self::regs();
            write_volatile(&mut (*regs).st_csr, 0);
            write_volatile(&mut (*regs).st_rvr, reload & SYSTICK_RVR_MAX);
            write_volatile(&mut (*regs).st_cvr, 0);

            let mut control = 1 << SYSTICK_CSR_ENABLE_BIT;
            if let ClockSource::Core = clk_source {
                control |= 1 << SYSTICK_CSR_CLKSOURCE_BIT;
            }

            write_volatile(&mut (*regs).st_csr, control);
        }
    }

    /// Delay in microseconds
    pub fn delay_us(&mut self, us: u32, core_clk_mhz: u32) {
        let ticks = core_clk_mhz * us;
        self.delay_ticks(ticks);
    }

    /// Delay in milliseconds
    pub fn delay_ms(&mut self, ms: u32, core_clk_mhz: u32) {
        self.delay_us(ms * 1_000, core_clk_mhz);
    }

    /// Delay in core ticks
    pub fn delay_ticks(&mut self, ticks: u32) {
        unsafe {
            let regs = Self::regs();

            write_volatile(&mut (*regs).st_csr, 0);
            write_volatile(&mut (*regs).st_rvr, (ticks - 1) & SYSTICK_RVR_MAX);
            write_volatile(&mut (*regs).st_cvr, 0);
            write_volatile(
                &mut (*regs).st_csr,
                (1 << SYSTICK_CSR_ENABLE_BIT) | (1 << SYSTICK_CSR_CLKSOURCE_BIT),
            );

            while read_volatile(&(*regs).st_csr) & (1 << SYSTICK_CSR_COUNTFLAG_BIT) == 0 {}

            write_volatile(&mut (*regs).st_csr, 0);
        }
    }

    pub fn current(&self) -> u32 {
        unsafe { read_volatile(&(*Self::regs()).st_cvr) }
    }

    pub fn has_wrapped(&self) -> bool {
        unsafe {
            (read_volatile(&(*Self::regs()).st_csr) & (1 << SYSTICK_CSR_COUNTFLAG_BIT)) != 0
        }
    }

    /// Initialize interrupt to fire every `us` microseconds
    pub fn init_systic_interrupt_us(&mut self, interval_us: u32, core_clk_mhz: u32) {
        let ticks = core_clk_mhz * interval_us;
        self.configure_interrupt_ticks(ticks);
    }

    /// Initialize interrupt to fire every `ms` milliseconds
    pub fn init_systic_interrupt_ms(&mut self, interval_ms: u32, core_clk_mhz: u32) {
        let ticks = core_clk_mhz * 1_000 * interval_ms;
        self.configure_interrupt_ticks(ticks);
    }

    /// Shared helper to configure interrupt-based ticking
    fn configure_interrupt_ticks(&mut self, ticks: u32) {
        unsafe {
            let regs = Self::regs();

            write_volatile(&mut (*regs).st_csr, 0);
            write_volatile(&mut (*regs).st_rvr, (ticks - 1) & SYSTICK_RVR_MAX);
            write_volatile(&mut (*regs).st_cvr, 0);

            let control = (1 << SYSTICK_CSR_ENABLE_BIT)
                | (1 << SYSTICK_CSR_CLKSOURCE_BIT)
                | (1 << SYSTICK_CSR_TICKINT_BIT);

            write_volatile(&mut (*regs).st_csr, control);
        }
    }
}





