use crate::{math::Vector2i, message_thread::MessageThreadSharedState};
use std::sync::atomic::Ordering;

impl MessageThreadSharedState {
    /// Get the current position of the window
    pub fn position(&self) -> Vector2i {
        let position = self.size.load(Ordering::Acquire);
        Vector2i::new((position & 0xFFFFFFFF) as i32, (position >> 32) as i32)
    }

    /// Set the reported position of the window
    pub fn set_position(&self, position: Vector2i) {
        self.position.store(
            position.x as u64 | ((position.y as u64) << 32),
            Ordering::Release,
        );
    }
}
