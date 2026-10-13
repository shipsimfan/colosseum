use crate::render::{FixedRenderObjects, RenderData, RenderJob};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use std::sync::Arc;

impl<'surface> RenderJob<'surface> {
    /// Get the device to use for rendering
    pub fn device(&self) -> &VulkanDevice {
        match self {
            RenderJob::Rendering { device, .. } => device,
            RenderJob::RecreateSwapchain { device, .. } => device,
        }
    }

    /// Get the memory properties of the adapter to use for rendering
    pub fn memory_properties(&self) -> &Arc<VulkanAdapterMemoryProperties> {
        match self {
            RenderJob::Rendering { device, .. } => device.memory_properties(),
            RenderJob::RecreateSwapchain { device, .. } => device.memory_properties(),
        }
    }

    /// Get the fixed render objects to use for rendering
    pub fn fixed_render_objects(&self) -> &Arc<FixedRenderObjects> {
        match self {
            RenderJob::Rendering { device, .. } => device.fixed_render_objects(),
            RenderJob::RecreateSwapchain { device, .. } => device.fixed_render_objects(),
        }
    }

    /// Get a mutable reference to the render data to use for rendering
    pub fn render_data(&mut self) -> &mut RenderData {
        match self {
            RenderJob::Rendering { device, .. } => device.render_data(),
            RenderJob::RecreateSwapchain { device, .. } => device.render_data(),
        }
    }
}
