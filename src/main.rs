#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::{ExceptionFrame, entry, exception};
use defmt as _;
use stm32h5::stm32h533;

#[entry]
fn main() -> ! {
    // Access to Cortex-M peripherals
    let _cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    // Access to chip-specific peripherals
    let _h5_peripherals = stm32h533::Peripherals::take().unwrap();

    #[allow(clippy::empty_loop)]
    loop {
    }
}

#[exception(trampoline = true)]
unsafe fn HardFault(_info: &ExceptionFrame) -> ! {
    // TODO: Implement logging
    #[allow(clippy::empty_loop)]
    loop { }
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) -> ! {
    // TODO: Implement logging
    #[allow(clippy::empty_loop)]
    loop { }
}