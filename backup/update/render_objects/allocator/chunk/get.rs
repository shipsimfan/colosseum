use crate::update::render_objects::allocator::GpuMemoryChunk;

impl GpuMemoryChunk {
    /// Get the largest available free block size in this chunk, if there is one
    pub fn largest_free_block_size(&self) -> Option<usize> {
        self.largest_free_block
    }
}
