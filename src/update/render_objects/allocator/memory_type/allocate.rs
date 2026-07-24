use crate::{
    Result,
    update::{GpuAllocatedMemory, render_objects::allocator::GpuMemoryType},
};

impl GpuMemoryType {
    /// Allocate a new block of GPU memory from this memory type
    pub fn allocate(&mut self, block_index: usize) -> Result<GpuAllocatedMemory> {
        todo!()
    }
}
