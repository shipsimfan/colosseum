use alexandria::gpu::VulkanDeviceMemory;
use std::sync::Arc;

mod bind;
mod get;
mod new;

pub(crate) struct GpuAllocatedMemory {
    /// The Vulkan device memory that this allocated memory is using
    device_memory: Arc<VulkanDeviceMemory>,

    /// The index of the memory type that this allocated memory is using
    ///
    /// If this is [`None`], then this is a dedicated memory allocation
    memory_type_index: Option<u8>,

    /// The index of the free list to insert this block into
    size: u8,

    /// The index of the chunk that this allocated memory is using
    chunk_index: u16,

    /// The offset into the device memory that this allocated memory is using
    offset: u32,
}
