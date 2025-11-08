use crate::{math::Vector2u, message_thread::MessageThreadSharedState};
use std::sync::atomic::Ordering;

impl MessageThreadSharedState {
    /// Set the reported size of the window
    pub fn set_size(&self, size: Vector2u) {
        self.size
            .store(size.x as u64 | ((size.y as u64) << 32), Ordering::Release);
    }
}
