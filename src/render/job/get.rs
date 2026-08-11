use crate::render::RenderJob;
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanFormat};
use std::sync::Arc;

impl<'surface> RenderJob<'surface> {
    /// Get the device to use for rendering
    pub fn device(&self) -> &VulkanDevice {
        match self {
            RenderJob::Rendering { device, .. } => device,
            RenderJob::RecreateSwapchain { device, .. } => device,
        }
    }

    /// Get the swapchain format to use for rendering
    pub fn swapchain_format(&self) -> VulkanFormat {
        match self {
            RenderJob::Rendering { device, .. } => device.swapchain_format(),
            RenderJob::RecreateSwapchain { device, .. } => device.swapchain_format(),
        }
    }

    /// Get the memory properties of the adapter to use for rendering
    pub fn memory_properties(&self) -> &Arc<VulkanAdapterMemoryProperties> {
        match self {
            RenderJob::Rendering { device, .. } => device.memory_properties(),
            RenderJob::RecreateSwapchain { device, .. } => device.memory_properties(),
        }
    }

    pub fn render_data(&mut self) -> &mut crate::render::RenderData {
        match self {
            RenderJob::Rendering { device, .. } => device.render_data(),
            RenderJob::RecreateSwapchain { device, .. } => device.render_data(),
        }
    }
}
