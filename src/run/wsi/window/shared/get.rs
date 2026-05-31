use crate::run::wsi::{SharedWindow, window::shared::decode_size};
use alexandria::math::Vector2u;
use std::sync::atomic::Ordering;

impl SharedWindow {
    /// Get the size of the window
    pub(in crate::run::wsi::window) fn size(&self) -> Vector2u {
        decode_size(self.size.load(Ordering::Acquire))
    }
}
