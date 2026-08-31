use crate::{
    logging::Logger,
    render::{FixedRenderObjects, RenderData, job::GraphicsDevice},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanFormat};
use std::sync::Arc;

impl GraphicsDevice {
    /// Get the logger for this graphics device
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Get the format for swapchain images that this graphics device supports
    pub fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }

    /// Get the memory properties of the adapter for this graphics device
    pub fn memory_properties(&self) -> &Arc<VulkanAdapterMemoryProperties> {
        &self.memory_properties
    }

    /// Get the fixed render objects for this graphics device
    pub fn fixed_render_objects(&self) -> &Arc<FixedRenderObjects> {
        self.render_objects.fixed()
    }

    /// Get the queue family index for the render queue of this graphics device
    pub fn render_queue_family(&self) -> u32 {
        self.queue.queue_family()
    }

    /// Get the current render data for the frame
    pub fn render_data(&mut self) -> &mut RenderData {
        &mut self.render_data[self.render_data_index]
    }
}
