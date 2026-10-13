use crate::update::render_objects::allocator::GpuMemoryChunk;
use alexandria::gpu::VulkanDevice;

mod allocate;
mod free;
mod new;
mod supports;

/// A set of chunks on a specific memory type that can be used to allocate GPU resources
pub(in crate::update::render_objects::allocator) struct GpuMemoryType {
    /// The chunks of GPU memory that have been allocated
    chunks: Vec<GpuMemoryChunk>,

    /// The memory type index that this memory type corresponds to
    memory_type_index: u32,

    /// The size of each chunk of GPU memory
    chunk_size: u32,

    /// The minimum size of a block of GPU memory that can be allocated
    min_block_size: u32,

    /// The maximum size of a block of GPU memory that can be allocated
    max_block_size: u32,

    /// The index of this memory type in the allocator's list of memory types
    index: u8,

    /// The device to allocate GPU memory from
    device: VulkanDevice,
}
