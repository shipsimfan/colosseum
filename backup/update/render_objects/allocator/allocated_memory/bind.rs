use crate::{Error, Result, update::render_objects::GpuAllocatedMemory};
use alexandria::gpu::VulkanBuffer;

impl GpuAllocatedMemory {
    /// Bind this allocated memory to the given Vulkan buffer
    pub fn bind_buffer(&self, buffer: &mut VulkanBuffer, offset: u32) -> Result<()> {
        buffer
            .bind_memory(&self.device_memory, (self.offset + offset) as _)
            .map_err(Error::new_inner)
    }
}
