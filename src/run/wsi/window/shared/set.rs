use crate::run::wsi::{SharedWindow, window::shared::encode_size};
use alexandria::math::Vector2u;
use std::sync::atomic::Ordering;

impl SharedWindow {
    /// Set the size of the window
    pub fn set_size(&self, size: Vector2u) {
        self.size.store(encode_size(size), Ordering::Release);
    }
}
