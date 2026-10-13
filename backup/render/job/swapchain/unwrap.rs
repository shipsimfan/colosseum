use crate::{
    Error, Result,
    render::job::{GraphicsDevice, Swapchain},
};
use alexandria::gpu::VulkanSurface;

impl<'surface> Swapchain<'surface> {
    /// Unwrap the swapchain, returning the graphics device
    pub fn unwrap(mut self, device: &GraphicsDevice) -> Result<&'surface mut VulkanSurface> {
        device.wait_idle().map_err(Error::new_inner)?;
        Ok(self.swapchain.take().unwrap().unwrap_surface())
    }
}
