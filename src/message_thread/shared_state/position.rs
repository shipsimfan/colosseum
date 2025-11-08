use crate::{math::Vector2i, message_thread::MessageThreadSharedState};
use std::sync::atomic::Ordering;

impl MessageThreadSharedState {
    /// Set the reported position of the window
    pub fn set_position(&self, position: Vector2i) {
        self.position.store(
            position.x as u64 | ((position.y as u64) << 32),
            Ordering::Release,
        );
    }
}
