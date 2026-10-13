use crate::{
    Result, ThreadManager,
    logging::Logger,
    render::{
        GpuTransferQueue, RenderJob,
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
        thread_manager: &ThreadManager,
    ) -> Result<(RenderJob<'surface>, GpuTransferQueue)> {
        let (mut device, transfer_queue) =
            GraphicsDevice::new(adapter, instance, surface, logger, thread_manager)?;

        let swapchain = Swapchain::new(surface, size, &mut device)?;

        Ok((RenderJob::Rendering { device, swapchain }, transfer_queue))
    }
}
