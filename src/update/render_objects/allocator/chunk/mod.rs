use alexandria::gpu::VulkanDeviceMemory;
use std::sync::Arc;

mod new;

/// A single fixed-size chunk of GPU memory that can be used to allocate resources from
pub(in crate::update::render_objects::allocator) struct GpuMemoryChunk {
    /// The memory used by this chunk
    memory: Arc<VulkanDeviceMemory>,

    /// The index of the largest available free block in this chunk
    largest_free_block: usize,

    /// The free lists for this chunk
    free_lists: Box<[Vec<u32>]>,
}
