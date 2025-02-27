use heapless::Vec;


// Requirements for accessing a subsystem
pub enum AccessRequirement {
    FullExclusive, // Needs full and sole access to the subsystem
    NotNeeded,     // Doesn't use the subsystem
    Shared,        // Can run with other events using the subsystem
}

struct EventRequirement<'a> {
    subsystem: &'a str,  // Name of the subsystem
    access: AccessRequirement,
}

struct Event<'a> {
    name: &'a str,
    requirements: Vec<EventRequirement<'a>>,
}


pub struct EventManager<'a> {
    events: Vec<&'a mut dyn Event, 8>, // Supports up to 8 events
}

impl<'a> EventManager<'a> {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn register_event(&mut self, event: &'a mut dyn Event) {
        self.events.push(event).ok(); // Ignore overflow
    }

    pub fn check_events(&mut self) {
        for event in self.events.iter_mut() {
            if event.check_condition() {
                event.trigger_action();
            }
        }
    }
}
