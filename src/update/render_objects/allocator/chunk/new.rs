use crate::update::render_objects::allocator::GpuMemoryChunk;
use alexandria::gpu::VulkanDeviceMemory;
use std::sync::Arc;

impl GpuMemoryChunk {
    /// Creates a new [`GpuMemoryChunk`] with the given memory and free lists
    pub fn new(
        memory: VulkanDeviceMemory,
        size: u32,
        min_block_size: u32,
        max_block_size: u32,
    ) -> GpuMemoryChunk {
        let memory = Arc::new(memory);

        let min_block_size_log2 = min_block_size.trailing_zeros();
        let max_block_size_log2 = max_block_size.trailing_zeros();
        let num_free_lists = (max_block_size_log2 - min_block_size_log2 + 1) as usize;
        let mut free_lists = vec![Vec::new(); num_free_lists].into_boxed_slice();

        for i in 0..size / max_block_size {
            free_lists[num_free_lists - 1].push(i * max_block_size);
        }

        GpuMemoryChunk {
            memory,
            largest_free_block: Some(num_free_lists - 1),
            free_lists,
        }
    }
}
