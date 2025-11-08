use crate::{
    MessageThread,
    math::{Vector2i, Vector2u},
};

impl MessageThread {
    /// Get the size of the window
    pub fn window_size(&self) -> Vector2u {
        self.shared_state.size()
    }

    /// Get the position of the window
    pub fn window_position(&self) -> Vector2i {
        self.shared_state.position()
    }
}
