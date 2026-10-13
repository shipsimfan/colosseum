use crate::update::{Inputs, input::MAX_KEYS};

impl Inputs {
    /// Create a new set of [`Inputs`]
    pub(in crate::update) fn new() -> Inputs {
        Inputs {
            keys: [false; MAX_KEYS],
            keys_down: [false; MAX_KEYS],
            keys_up: [false; MAX_KEYS],
        }
    }
}
