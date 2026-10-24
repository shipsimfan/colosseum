use crate::{Key, update::Input};

impl Input {
    /// Add a key to the set of currently pressed keys
    pub(in crate::update::input) fn set_key_down(&mut self, key: Key) {
        if !self.key(key) {
            self.keys_down[key as usize] = true;
        }

        self.keys[key as usize] = true;
    }

    /// Remove a key from the set of currently pressed keys
    pub(in crate::update::input) fn set_key_up(&mut self, key: Key) {
        if self.key(key) {
            self.keys_up[key as usize] = true;
        }

        self.keys[key as usize] = false;
    }
}
