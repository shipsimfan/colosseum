use crate::{
    Error, Result,
    render::job::{GraphicsDevice, Swapchain},
};
use alexandria::{
    gpu::VulkanSwapchainPresentMode,
    math::{Vector2i, Vector2u},
};

impl<'surface> Swapchain<'surface> {
    /// Create a new [`Swapchain`] from a [`GraphicsDevice`]
    pub fn new(device: GraphicsDevice<'surface>, size: Vector2u) -> Result<Swapchain<'surface>> {
        let swapchain = device
            .device()
            .create_swapchain(
                3,
                device.swapchain_format(),
                Vector2i::new(size.x as _, size.y as _),
                VulkanSwapchainPresentMode::Fifo,
                device.surface(),
            )
            .map_err(Error::new_inner)?;

        Ok(Swapchain { device, swapchain })
    }
}
