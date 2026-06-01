use crate::{Key, update::Inputs};

impl Inputs {
    /// Get if a key is currently pressed
    pub fn key(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    /// Get if a key was just pressed this frame
    pub fn key_down(&self, key: Key) -> bool {
        self.keys_down[key as usize]
    }

    /// Get if a key was just released this frame
    pub fn key_up(&self, key: Key) -> bool {
        self.keys_up[key as usize]
    }
}
