use crate::update::render_objects::GpuAllocatedMemory;
use std::sync::Arc;

impl GpuAllocatedMemory {
    /// Get the Vulkan device memory that this allocated memory is using
    pub fn device_memory(&self) -> &Arc<alexandria::gpu::VulkanDeviceMemory> {
        &self.device_memory
    }

    /// Get the index of the memory type that this allocated memory is using
    ///
    /// If this is [`None`], then this is a dedicated memory allocation
    pub(in crate::update::render_objects::allocator) fn memory_type_index(&self) -> Option<u8> {
        self.memory_type_index
    }

    /// Get the index of the free list to insert this block into
    pub(in crate::update::render_objects::allocator) fn size(&self) -> u8 {
        self.size
    }

    /// Get the index of the chunk that this allocated memory is using
    pub(in crate::update::render_objects::allocator) fn chunk_index(&self) -> u16 {
        self.chunk_index
    }

    /// Get the offset into the device memory that this allocated memory is using
    pub(in crate::update::render_objects::allocator) fn offset(&self) -> u32 {
        self.offset
    }
}
