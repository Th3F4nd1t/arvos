pub trait Event {
    fn check_condition(&mut self) -> bool;
    fn trigger_action(&mut self);
}