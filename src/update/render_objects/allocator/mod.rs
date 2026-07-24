use alexandria::gpu::VulkanDevice;
use chunk::GpuMemoryChunk;
use std::sync::Arc;

mod chunk;

mod new;

/// An allocator for Vulkan resources on the GPU
pub(in crate::update::render_objects) struct GpuAllocator {
    /// The chunks of GPU memory that have been allocated
    chunks: Vec<GpuMemoryChunk>,

    /// The size of each chunk of GPU memory
    chunk_size: u64,

    /// The minimum size of a block of GPU memory that can be allocated
    min_block_size: u64,

    /// The maximum size of a block of GPU memory that can be allocated
    max_block_size: u64,

    /// The device to allocate GPU memory from
    device: Arc<VulkanDevice>,

    /// The memory type index to use when allocating GPU memory
    memory_type_index: u32,
}
