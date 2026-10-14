use crate::render::GraphicsDevice;
use alexandria::gpu::VulkanFormat;
use colosseum_core::Logger;

impl GraphicsDevice {
    /// Get the logger for this graphics device
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Get the format for swapchain images that this graphics device supports
    pub fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }
}
