use crate::update::GpuAllocatedMemory;
use alexandria::gpu::VulkanDeviceMemory;
use std::sync::Arc;

impl GpuAllocatedMemory {
    /// Create a new [`GpuAllocatedMemory`]
    pub(in crate::update::render_objects::allocator) fn new(
        device_memory: Arc<VulkanDeviceMemory>,
        memory_type_index: u8,
        size: u8,
        chunk_index: u16,
        offset: u32,
    ) -> GpuAllocatedMemory {
        GpuAllocatedMemory {
            device_memory,
            memory_type_index: Some(memory_type_index),
            size,
            chunk_index,
            offset,
        }
    }

    /// Create a new [`GpuAllocatedMemory`] for a dedicated allocation
    pub(in crate::update::render_objects::allocator) fn new_dedicated(
        device_memory: Arc<VulkanDeviceMemory>,
    ) -> GpuAllocatedMemory {
        GpuAllocatedMemory {
            device_memory,
            memory_type_index: None,
            size: 0,
            chunk_index: 0,
            offset: 0,
        }
    }
}
