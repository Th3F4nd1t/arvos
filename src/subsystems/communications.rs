use std::collections::VecDeque;
use crate::status_codes::StatusCodes; // this needs to be in main.rs


struct Communications {
    current_codes: VecDeque<StatusCodes>
}

impl Communications {
    fn new() -> Self {
        Self { 
            current_codes: VecDeque::new()
        }
    }

    fn add_status_code(&mut self, code: StatusCodes) {
        self.current_codes.push_back(code);
    }

    fn update_comms(&mut self) {
        // Implement the logic for updating communications here
    }
}