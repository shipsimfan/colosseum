use crate::render::{GpuAllocatedMemory, public_data::assets::allocator::GpuMemoryType};

impl GpuMemoryType {
    /// Free a block of GPU memory that was previously allocated
    pub fn free(&mut self, memory: GpuAllocatedMemory) {
        self.chunks[memory.chunk_index() as usize].free(memory);
    }
}
