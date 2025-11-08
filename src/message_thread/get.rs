use crate::{MessageThread, math::Vector2u};

impl MessageThread {
    /// Get the size of the window
    pub fn window_size(&self) -> Vector2u {
        self.shared_state.size()
    }
}
