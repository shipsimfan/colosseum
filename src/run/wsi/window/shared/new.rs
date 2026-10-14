use crate::run::wsi::{
    SharedWindow,
    window::shared::{encode_position, encode_size},
};
use alexandria::math::{Vector2i, Vector2u};
use std::sync::atomic::{AtomicBool, AtomicU64};

impl SharedWindow {
    /// Create a new [`SharedWindow`] state
    pub(in crate::run::wsi) fn new(
        position: Vector2i,
        size: Vector2u,
        fullscreen: bool,
        maximized: bool,
    ) -> SharedWindow {
        SharedWindow {
            position: AtomicU64::new(encode_position(position)),
            size: AtomicU64::new(encode_size(size)),
            fullscreen: AtomicBool::new(fullscreen),
            maximized: AtomicBool::new(maximized),
        }
    }
}
