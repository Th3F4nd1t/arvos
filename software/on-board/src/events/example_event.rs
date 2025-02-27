use crate::subsystems::example_subsystem::ExampleSubsystem;
use crate::events::event_trait::Event;

pub struct ExampleEvent {
    threshold: u16,
    subsystem: ExampleSubsystem,
}

impl ExampleEvent {
    pub fn new(threshold: u16, subsystem: ExampleSubsystem) -> Self {
        Self { threshold, subsystem }
    }
}

impl Event for ExampleEvent {
    fn check_condition(&mut self) -> bool {
        return true;
    }

    fn trigger_action(&mut self) {
        self.subsystem.toggle_led();
    }

    fn get_requirements(&self) {}
}
