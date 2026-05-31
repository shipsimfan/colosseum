use crate::{
    Error, Result,
    render::job::{GraphicsDevice, Swapchain},
};

impl<'surface> Swapchain<'surface> {
    /// Unwrap the swapchain, returning the graphics device
    pub fn unwrap(self) -> Result<GraphicsDevice<'surface>> {
        self.device.wait_idle().map_err(Error::new_inner)?;
        Ok(self.device)
    }
}
