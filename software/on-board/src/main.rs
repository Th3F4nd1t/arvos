#![no_std]
#![no_main]

mod events;
mod subsystems;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    loop {

    }
}
