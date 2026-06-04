use crate::{
    Result,
    logging::Logger,
    render::{
        RenderJob,
        job::{GraphicsDevice, Swapchain},
    },
};
use alexandria::{
    gpu::{VulkanInstance, VulkanSurface},
    math::Vector2u,
};

impl<'surface> RenderJob<'surface> {
    /// Create a new render job by locating a graphics device and creating a swapchain
    pub fn new(
        adapter: Option<&str>,
        instance: &VulkanInstance,
        surface: &'surface mut VulkanSurface,
        size: Vector2u,
        logger: &Logger,
    ) -> Result<RenderJob<'surface>> {
        let mut device = GraphicsDevice::new(adapter, instance, surface, logger)?;

        let swapchain = Swapchain::new(surface, size, &mut device)?;

        Ok(RenderJob::Rendering { device, swapchain })
    }
}
