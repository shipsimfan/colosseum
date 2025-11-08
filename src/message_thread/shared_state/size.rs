use crate::{math::Vector2u, message_thread::MessageThreadSharedState};
use std::sync::atomic::Ordering;

impl MessageThreadSharedState {
    pub fn size(&self) -> Vector2u {
        let size = self.size.load(Ordering::Acquire);
        Vector2u::new((size & 0xFFFFFFFF) as u32, (size >> 32) as u32)
    }

    /// Set the reported size of the window
    pub fn set_size(&self, size: Vector2u) {
        self.size
            .store(size.x as u64 | ((size.y as u64) << 32), Ordering::Release);
    }
}
