#[allow(dead_code)]
pub enum StatusCodes {
    // General
    NoCode = 000,

    // Pre Launch
    PoweredOn = 100,
    RunningSystemsCheck = 101,
    AwaitingUserConfirmation = 102,
    ReadyForInit = 103
}