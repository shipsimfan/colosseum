use crate::run::wsi::{
    SharedWindow,
    window::shared::{encode_position, encode_size},
};
use alexandria::math::{Vector2i, Vector2u};
use std::sync::atomic::Ordering;

impl SharedWindow {
    /// Set the position of the window
    pub(in crate::run::wsi) fn set_position(&self, position: Vector2i) {
        self.position
            .store(encode_position(position), Ordering::Release);
    }

    /// Set the size of the window
    pub(in crate::run::wsi) fn set_size(&self, size: Vector2u) {
        self.size.store(encode_size(size), Ordering::Release);
    }

    /// Set whether the window is currently fullscreen or not
    pub(in crate::run::wsi) fn set_fullscreen(&self, fullscreen: bool) {
        self.fullscreen.store(fullscreen, Ordering::Release);
    }

    /// Set whether the window is currently maximized or not
    pub(in crate::run::wsi) fn set_maximized(&self, maximized: bool) {
        self.maximized.store(maximized, Ordering::Release);
    }

    /// Set whether the window is currently focused or not
    pub(in crate::run::wsi) fn set_focused(&self, focused: bool) {
        self.focused.store(focused, Ordering::Release);
    }

    /// Set the current position of the mouse cursor
    pub(in crate::run::wsi) fn set_mouse_position(&self, position: Vector2i) {
        self.mouse_position
            .store(encode_position(position), Ordering::Release);
    }

    /// Add to the current scroll wheel delta
    pub(in crate::run::wsi) fn add_mouse_wheel(&self, delta: i32) {
        self.mouse_wheel.fetch_add(delta, Ordering::Release);
    }
}
