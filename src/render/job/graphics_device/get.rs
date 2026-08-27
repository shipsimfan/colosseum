use crate::{
    logging::Logger,
    render::{
        FixedRenderObjects, RenderData,
        job::{GraphicsDevice, RenderToken},
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanFormat};
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

    /// Get the command buffer for the given frame index
    pub fn command_buffer(&mut self, token: &RenderToken) -> &mut VulkanCommandBuffer {
        &mut self.command_pool[self.frame_data[token.frame_index()].command_buffer()]
    }

    /// Get the memory properties of the adapter for this graphics device
    pub fn memory_properties(&self) -> &Arc<VulkanAdapterMemoryProperties> {
        &self.memory_properties
    }

    /// Get the fixed render objects for this graphics device
    pub fn fixed_render_objects(&self) -> &Arc<FixedRenderObjects> {
        self.render_objects.fixed()
    }

    /// Get the current render data for the frame
    pub fn render_data(&mut self) -> &mut RenderData {
        self.frame_data[self.frame_index].render_data_mut()
    }
}
