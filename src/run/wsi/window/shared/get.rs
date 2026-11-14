use crate::run::wsi::{
    SharedWindow,
    window::shared::{decode_position, decode_size},
};
use alexandria::math::{Vector2i, Vector2u};
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

    /// Get whether the window is currently focused or not
    pub(in crate::run::wsi::window) fn focused(&self) -> bool {
        self.focused.load(Ordering::Acquire)
    }

    /// Get the current position of the mouse cursor
    pub(in crate::run::wsi::window) fn mouse_position(&self) -> Vector2i {
        decode_position(self.mouse_position.load(Ordering::Acquire))
    }

    /// Get the current scroll wheel delta
    pub(in crate::run::wsi::window) fn mouse_wheel(&self) -> i32 {
        self.mouse_wheel.swap(0, Ordering::Acquire)
    }
}
