use crate::{
    Error, Result,
    update::{
        GpuAllocatedMemory,
        render_objects::allocator::{GpuMemoryChunk, GpuMemoryType},
    },
};
#[cfg(debug_assertions)]
use std::ffi::CString;

impl GpuMemoryType {
    /// Allocate a new block of GPU memory from this memory type
    pub fn allocate(&mut self, block_index: usize, name: &str) -> Result<GpuAllocatedMemory> {
        // Check for a chunk that can fit the requested block size
        for (index, chunk) in self.chunks.iter_mut().enumerate() {
            if let Some(largest_free_block_size) = chunk.largest_free_block_size() {
                if largest_free_block_size >= block_index {
                    return chunk.allocate(block_index, self.index, index as _);
                }
            }
        }

        // If no chunk can fit the requested block size, create a new chunk and allocate from it
        let mut memory = self
            .device
            .allocate_memory(self.chunk_size as u64, self.memory_type_index as _)
            .map_err(Error::new_inner)?;
        let index = self.chunks.len() as _;

        #[cfg(debug_assertions)]
        let name = CString::new(format!(
            "{} Chunk {:X}-{}",
            name, self.memory_type_index, index
        ))
        .unwrap();
        #[cfg(debug_assertions)]
        self.device
            .set_object_name(&mut memory, &name)
            .map_err(Error::new_inner)?;

        self.chunks.push(GpuMemoryChunk::new(
            memory,
            self.chunk_size,
            self.min_block_size,
            self.max_block_size,
        ));

        self.chunks
            .last_mut()
            .unwrap()
            .allocate(block_index, self.index, index)
    }
}
