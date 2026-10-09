use crate::{
    Result,
    run::wsi::{
        SharedWindow,
        window::shared::{encode_position, encode_size},
    },
};
use alexandria::{
    Notify,
    math::{Vector2i, Vector2u},
};
use colosseum_core::new_error;
use std::sync::atomic::{AtomicBool, AtomicU64};

impl SharedWindow {
    /// Create a new [`SharedWindow`] state
    pub(in crate::run::wsi) fn new(
        position: Vector2i,
        size: Vector2u,
        fullscreen: bool,
        maximized: bool,
    ) -> Result<SharedWindow> {
        let restored_notify = Notify::new(true, false)
            .map_err(|error| new_error!("unable to create the restore notify - {}", error))?;

        Ok(SharedWindow {
            position: AtomicU64::new(encode_position(position)),
            size: AtomicU64::new(encode_size(size)),
            fullscreen: AtomicBool::new(fullscreen),
            maximized: AtomicBool::new(maximized),
            restored_notify,
        })
    }
}
