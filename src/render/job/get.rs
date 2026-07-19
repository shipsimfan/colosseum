use crate::render::RenderJob;
use alexandria::gpu::{VulkanDevice, VulkanFormat};

impl<'surface> RenderJob<'surface> {
    /// Get the device to use for rendering
    pub(crate) fn device(&self) -> &VulkanDevice {
        match self {
            RenderJob::Rendering { device, .. } => device,
            RenderJob::RecreateSwapchain { device, .. } => device,
        }
    }

    /// Get the swapchain format to use for rendering
    pub(crate) fn swapchain_format(&self) -> VulkanFormat {
        match self {
            RenderJob::Rendering { device, .. } => device.swapchain_format(),
            RenderJob::RecreateSwapchain { device, .. } => device.swapchain_format(),
        }
    }

    /// Get the device local memory type index to use for rendering
    pub(crate) fn device_local_memory_type(&self) -> usize {
        match self {
            RenderJob::Rendering { device, .. } => device.device_local_memory_type(),
            RenderJob::RecreateSwapchain { device, .. } => device.device_local_memory_type(),
        }
    }
}
