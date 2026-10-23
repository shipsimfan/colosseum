use crate::{Result, render::GraphicsDevice};
use colosseum_core::new_error;

/// Stop any ongoing rendering operations for the given world for clean shutdown
pub(crate) fn stop_render(device: &GraphicsDevice) -> Result<()> {
    device
        .wait_idle()
        .map_err(|error| new_error!("unable to wait for the graphics device to idle - {}", error))
}
