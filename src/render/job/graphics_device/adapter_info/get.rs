use crate::render::job::graphics_device::VulkanAdapterInfo;
use alexandria::gpu::VulkanFormat;

impl<'instance> VulkanAdapterInfo<'instance> {
    /// Get the format for swapchain images that this adapter supports
    pub fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }

    /// Get the index of the graphics queue family to use with this adapter
    pub fn graphics_queue_family_index(&self) -> u32 {
        self.graphics_queue_family_index
    }
}
