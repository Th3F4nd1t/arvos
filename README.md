# Control Structure
You have subsystems, essentially the same as a subsystem in frc, it's a hardware abstraction layer. Providing commands on it's given hardware that other things can interface with.
You have events. An event is specified by a start condition, an end condition, a run command, and a list of requirements. A requirement is simply "this subsystem with this permission level." Permission levels could be:
<br><br>
In the main loop it checks if an event should be started. If so it spins up a thread of that event. It allows that thread to run until the end condition is met, in which case it terminates the thread, or runs an "end" function. If the thread takes more than a specified amount of time to end, it can either forcibly crash the thread, or so smth else depending on what is specified for that specific event.

# Files
A run down on the goal of each file and what is going to be in the file.

## on-board/

### on-board/main.rs
basically just the loop and some metacode; creates a rocket.rs instance and runs an init() and a loop() or similar
- Main Loop
- Watchdog timers
- Safe mode (disabled)

### on-board/rocket.rs
where subsytems are created, where events are made, the event manager is setup, etc
- Power-on self-test (POST)
    - Sensor validation


## on-board/events/

### on-board/events/mod.rs
mod basically just has a `pub mod example_subsystem;` for each subsystem. basically makes that stuff accessible from other files.

### on-board/events/event_base.rs
- Trait `EventBase`
    - Construction: pass in refernces to the subsytesm required.
    - Fn `startCondition()`
    - Fn `endCondition()`
    - Fn `execute()`
    - Fn `getRequirements() -> EventRequirements`

### on-board/events/event_manager.rs
- Struct `EventManager`
- Impl `EventManager`
- Struct `EventRequirements`
- Struct `EventRequirement`
- Enum `AccessLevel`
    - no access: not needed
    - regular access: commands will be run, not high level commands though
    - sudo access: all commands can be run
    - regular exclusive: only that event can use the subsystem and blocks the rest of the events from doing anything l
    - sudo exclusive: blocked and high level access

### on-board/events/example_event.rs
- Struct `ExampleEvent`
- Impl `ExampleEvent` for `EventBase`


## on-board/subsystems/

### on-board/subsystems/mod.rs
same as other mod.rs but with subsystems

### on-board/subsystems/subsystem_base.rs
just a template and some meta stuff that will help with other subsystems
<br> Also need to figure out how to do access level stuff for locking a subsystem. that should be done probably

- Trait `SubsystemBase`
    - Fn `init()`: Runs on rocket startup
    - Fn `onEventStart()`: Runs when an event starts

### on-board/subsystems/example_subsystem.rs
- Struct `ExampleSubsystem`
    - Fn `exampleFunction(u32) -> bool`: does this
- Impl `ExampleSubsystem` for `SubsystemBase`


## on-board/utils/

### on-board/utils/mod.rs
another one of these

### on-board/utils/constants.rs
constants, eg:
```rs
pub const MAX_THRUST: f32 = 10000000;
pub const BAUD_RATE: u32 = 115200;
pub const ROCKET_NAME: &str = "TBD";
```

### on-board/utils/util.rs
anything that's used more than once across files, so prolly basic pin access, etc


## ground-station

### ground-station/src/

#### ground-station/src/main.py
- Communications
- Backup comms
- Passing to remote
- Passing to on-board
- Logging
- Platform mechanical
- Network handling
- Error handling


## remote

### remote/src/

#### remote/src/main.py
- Sending/recieving
- Network handling
- Error handling

#### remote/src/gui.py
- Dashboard


# Other Notes
- on-board
- Designing events
- Designing subsystems
- Actually implementing subsystems
- Actually implementing events
- Communications
    - Protocol design (message format, packet handling, retries)
    - Error correction (checksums, CRC)