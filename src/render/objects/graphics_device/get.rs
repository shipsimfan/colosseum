use crate::render::GraphicsDevice;
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanFormat};
use colosseum_core::Logger;

impl GraphicsDevice {
    /// Get the logger for this graphics device
    pub(in crate::render) fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Get the index of the render queue family for this graphics device
    pub(in crate::render) fn render_queue_family(&self) -> u32 {
        self.render_queue_family
    }

    /// Get the format for swapchain images that this graphics device supports
    pub(in crate::render) fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }

    /// Get the memory properties for this graphics device
    pub(in crate::render) fn memory_properties(&self) -> &VulkanAdapterMemoryProperties {
        &self.memory_properties
    }
}
