use crate::render::job::GraphicsDevice;
use alexandria::gpu::{VulkanDevice, VulkanFormat, VulkanSurface};

impl<'surface> GraphicsDevice<'surface> {
    /// Get the Vulkan device that this graphics device is using for rendering
    pub fn device(&self) -> &VulkanDevice {
        &self.device
    }

    /// Get the surface that this graphics device is rendering to
    pub fn surface(&self) -> &'surface VulkanSurface {
        self.surface
    }

    /// Get the format for swapchain images that this graphics device supports
    pub fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }
}
