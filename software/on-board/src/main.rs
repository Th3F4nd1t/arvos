#![no_std]
#![no_main]

mod events;
mod subsystems;

use crate::events::event_manager::EventManager;
use crate::events::example_event::ExampleEvent;
use crate::subsystems::example_subsystem::ExampleSubsystem;
use cortex_m::Peripherals;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    
    let subsystem = ExampleSubsystem::new(correct_led_pin);
    let mut example_event = ExampleEvent::new(1000, subsystem);
    
    let mut event_manager = EventManager::new();
    event_manager.register_event(&mut example_event);
    
    loop {
        event_manager.check_events();
        cortex_m::asm::delay(8_000_000);
    }
}
