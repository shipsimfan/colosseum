use crate::{logging::Logger, render::job::GraphicsDevice};
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
    pub fn command_buffer(&mut self, index: usize) -> &mut VulkanCommandBuffer {
        &mut self.command_pool[self.command_buffers[index]]
    }

    /// Get the memory properties of the adapter for this graphics device
    pub fn memory_properties(&self) -> &Arc<VulkanAdapterMemoryProperties> {
        &self.memory_properties
    }
}
