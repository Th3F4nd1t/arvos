## Todo
- on-board
    - Designing events
    - Designing subsystems
    - Actually implementing subsystems
    - Actually implementing events
    - Logging & telemetry for debugging

    - Diagnostics & Self-Checks
        - Power-on self-test (POST)
        - Sensor validation
    - Communications
        - Protocol design (message format, packet handling, retries)
        - Error correction (checksums, CRC)
- ground-station
    - Communications
    - Backup comms
    - Passing to remote
    - Passing to on-board
    - Logging
    - Platform mechanical
    - Network handling
    - Error handling
- remote
    - Dashboard
    - Sending/recieving
    - Network handling
    - Error handling

# Files

## on-board/

### on-board/main.rs
basically just the loop and some metacode; creates a rocket.rs instance and runs an init() and a loop() or similar
- Main Loop
- Watchdog timers
- Safe mode (disabled)
- 

### on-board/rocket.rs
where subsytems are created, where events are made, the event manager is setup, etc


## on-board/events/

### on-board/events/mod.rs
mod basically just has a `pub mod example_subsystem;` for each subsystem. basically makes that stuff accessible from other files.

### on-board/events/event_base.rs
- Class|Trait `EventBase`

### on-board/events/event_manager.rs
- Class `EventManager`
- Class `EventRequirements`
- Class `EventRequirement`
- Class `AccessLevel`

### on-board/events/example_event.rs
- Class `ExampleEvent`


## on-board/subsystems/

### on-board/subsystems/mod.rs
same as other mod.rs but with subsystems

### on-board/subsystems/subsystem_base.rs
just a template and some meta stuff that will help with other subsystems

### on-board/subsystems/example_subsystem.rs
- Class `ExampleSubsystem`


## on-board/utils/

### on-board/utils/mod.rs
another one of these

### on-board/utils/constants.rs
constants

### on-board/utils/util.rs
anything that's used more than once across files, so prolly basic pin access, etc


## ground-station

### ground-station/src/

#### ground-station/src/main.py
idk yet


## remote

### remote/src/

#### remote/src/main.py
idk yet

#### remote/src/gui.py
gui stuff