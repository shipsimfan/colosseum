use crate::update::{
    Input,
    input::{MAX_KEYS, MAX_MOUSE_BUTTONS},
};

impl Input {
    /// Reset the set of inputs for the next frame
    pub(in crate::update::input) fn reset(&mut self) {
        self.keys_down = [false; MAX_KEYS];
        self.keys_up = [false; MAX_KEYS];
        self.mouse_buttons_down = [false; MAX_MOUSE_BUTTONS];
        self.mouse_buttons_up = [false; MAX_MOUSE_BUTTONS];
    }
}
