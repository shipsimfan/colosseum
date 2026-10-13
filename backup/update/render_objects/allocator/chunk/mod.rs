use alexandria::gpu::VulkanDeviceMemory;
use std::sync::Arc;

mod allocate;
mod free;
mod get;
mod new;

/// A single fixed-size chunk of GPU memory that can be used to allocate resources from
pub(in crate::update::render_objects::allocator) struct GpuMemoryChunk {
    /// The memory used by this chunk
    memory: Arc<VulkanDeviceMemory>,

    /// The log2 of the minimum block size that can be allocated from this chunk
    min_block_size_log2: u32,

    /// The index of the largest available free block in this chunk
    largest_free_block: Option<usize>,

    /// The free lists for this chunk
    free_lists: Box<[Vec<u32>]>,
}
