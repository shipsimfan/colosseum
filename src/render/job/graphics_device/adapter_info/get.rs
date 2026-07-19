use crate::render::job::graphics_device::VulkanAdapterInfo;
use alexandria::{MemorySize, Uuid, gpu::VulkanFormat};

impl<'instance> VulkanAdapterInfo<'instance> {
    /// Get the name of this adapter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the UUID of this adapter
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Get the amount of device-local VRAM available on this adapter, in bytes
    pub fn device_local_vram(&self) -> MemorySize {
        self.device_local_vram
    }

    /// Get the format for swapchain images that this adapter supports
    pub fn swapchain_format(&self) -> VulkanFormat {
        self.swapchain_format
    }

    /// Get the index of the graphics queue family to use with this adapter
    pub fn graphics_queue_family_index(&self) -> u32 {
        self.graphics_queue_family_index
    }

    /// Get the index of the transfer queue family to use with this adapter
    pub fn transfer_queue_family_index(&self) -> u32 {
        self.transfer_queue_family_index
    }

    /// Get the memory type index for staging buffers that this adapter supports
    pub fn staging_buffer_memory_index(&self) -> usize {
        self.staging_buffer_memory_index
    }

    /// Get the memory type index for device-local buffers that this adapter supports
    pub fn device_local_buffer_memory_index(&self) -> usize {
        self.device_local_buffer_memory_index
    }
}
