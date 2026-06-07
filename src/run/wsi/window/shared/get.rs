use crate::run::wsi::{
    SharedWindow,
    window::shared::{decode_position, decode_size},
};
use alexandria::{
    Notify,
    math::{Vector2i, Vector2u},
};
use std::sync::atomic::Ordering;

impl SharedWindow {
    /// Get the position of the window
    pub(in crate::run::wsi::window) fn position(&self) -> Vector2i {
        decode_position(self.position.load(Ordering::Acquire))
    }

    /// Get the size of the window
    pub(in crate::run::wsi::window) fn size(&self) -> Vector2u {
        decode_size(self.size.load(Ordering::Acquire))
    }

    /// Get whether the window is currently fullscreen or not
    pub(in crate::run::wsi::window) fn fullscreen(&self) -> bool {
        self.fullscreen.load(Ordering::Acquire)
    }

    /// Get whether the window is currently maximized or not
    pub(in crate::run::wsi::window) fn maximized(&self) -> bool {
        self.maximized.load(Ordering::Acquire)
    }

    /// Get the restored notify for when the window has been restored from a minimized state
    pub fn restored_notify(&self) -> &Notify {
        &self.restored_notify
    }
}
