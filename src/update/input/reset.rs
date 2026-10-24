use crate::update::{Input, input::MAX_KEYS};

impl Input {
    /// Reset the set of inputs for the next frame
    pub(in crate::update::input) fn reset(&mut self) {
        self.keys_down = [false; MAX_KEYS];
        self.keys_up = [false; MAX_KEYS];
    }
}
