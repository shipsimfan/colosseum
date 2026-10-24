use crate::update::{Input, input::MAX_KEYS};

impl Input {
    /// Create a new set of [`Input`]
    pub(in crate::update) fn new() -> Input {
        Input {
            keys: [false; MAX_KEYS],
            keys_down: [false; MAX_KEYS],
            keys_up: [false; MAX_KEYS],
        }
    }
}
