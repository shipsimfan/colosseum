use crate::render::GraphicsDevice;
use alexandria::gpu::{VulkanFormat, VulkanQueue};
use colosseum_core::Logger;

impl GraphicsDevice {
    /// Get the logger for this graphics device
    pub(in crate::render) fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Get a mutable reference to the render queue for this graphics device
    pub(in crate::render) fn render_queue(&mut self) -> &mut VulkanQueue {
        &mut self.render_queue
    }

    /// Get the index of the render queue family for this graphics device
    pub(in crate::render) fn render_queue_family(&self) -> u32 {
        self.render_queue_family
    }

    /// Get the format for swapchain images that this graphics device supports
    pub(in crate::render) fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }
}
