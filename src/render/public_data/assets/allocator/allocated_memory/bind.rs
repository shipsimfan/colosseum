use crate::{Result, render::GpuAllocatedMemory};
use alexandria::gpu::VulkanBuffer;
use colosseum_core::new_error;

impl GpuAllocatedMemory {
    /// Bind this allocated memory to the given Vulkan buffer
    pub fn bind_buffer(&self, buffer: &mut VulkanBuffer, offset: u32) -> Result<()> {
        buffer
            .bind_memory(&self.device_memory, (self.offset + offset) as _)
            .map_err(|error| new_error!("unable to bind a buffer - {}", error))
    }
}
