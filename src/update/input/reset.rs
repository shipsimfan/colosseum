use crate::update::{Inputs, input::MAX_KEYS};

impl Inputs {
    /// Reset the set of inputs for the next frame
    pub(in crate::update) fn reset(&mut self) {
        self.keys_down = [false; MAX_KEYS];
        self.keys_up = [false; MAX_KEYS];
    }
}
