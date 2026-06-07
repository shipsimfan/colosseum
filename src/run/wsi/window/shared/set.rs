use crate::{
    Error, Result,
    run::wsi::{
        SharedWindow,
        window::shared::{encode_position, encode_size},
    },
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
    pub(in crate::run::wsi) fn set_size(&self, size: Vector2u) -> Result<()> {
        let encoded_size = encode_size(size);
        let old_value = self.size.swap(encoded_size, Ordering::Release);
        if old_value == 0 && encoded_size != 0 {
            self.restored_notify.notify().map_err(Error::new_inner)?;
        }
        Ok(())
    }

    /// Set whether the window is currently fullscreen or not
    pub(in crate::run::wsi) fn set_fullscreen(&self, fullscreen: bool) {
        self.fullscreen.store(fullscreen, Ordering::Release);
    }

    /// Set whether the window is currently maximized or not
    pub(in crate::run::wsi) fn set_maximized(&self, maximized: bool) {
        self.maximized.store(maximized, Ordering::Release);
    }
}
