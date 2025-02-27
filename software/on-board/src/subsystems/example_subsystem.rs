use stm32h7xx_hal::gpio::{Output, PushPull, Pin};

pub struct ExampleSubsystem {
    led: Pin<'A', 5, Output<PushPull>>,  // PA5 as LED output      // PC0 as ADC input
}

impl ExampleSubsystem {
    pub fn new(
        led: Pin<'A', 5, Output<PushPull>> // Pass ADC pin to struct
    ) -> Self {
        Self { led }
    }   

    pub fn toggle_led(&mut self) {
        self.led.toggle();
    }
}
