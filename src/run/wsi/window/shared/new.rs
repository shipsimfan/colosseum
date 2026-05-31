use crate::run::wsi::{SharedWindow, window::shared::encode_size};
use alexandria::math::Vector2u;
use std::sync::atomic::AtomicU64;

impl SharedWindow {
    /// Create a new [`SharedWindow`] state
    pub fn new(size: Vector2u) -> SharedWindow {
        SharedWindow {
            size: AtomicU64::new(encode_size(size)),
        }
    }
}
