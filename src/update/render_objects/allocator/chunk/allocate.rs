use crate::{
    Result,
    update::{GpuAllocatedMemory, render_objects::allocator::GpuMemoryChunk},
};

impl GpuMemoryChunk {
    /// Allocate a new block of GPU memory from this chunk
    pub fn allocate(
        &mut self,
        block_index: usize,
        memory_type_index: u8,
        chunk_index: u16,
    ) -> Result<GpuAllocatedMemory> {
        // Find a block that can fit the requested block size
        for i in block_index..=self.largest_free_block.unwrap() {
            if self.free_lists[i].len() == 0 {
                continue;
            }

            // Split the block into smaller blocks if necessary
            let mut j = i;
            while j > block_index {
                let offset = self.free_lists[j].pop().unwrap();
                let next_offset = offset + (1 << (j as u32 + self.min_block_size_log2 - 1));
                self.free_lists[j - 1].push(next_offset);
                self.free_lists[j - 1].push(offset);

                if j == self.largest_free_block.unwrap() && self.free_lists[j].len() == 0 {
                    self.largest_free_block = Some(j - 1);
                }

                j -= 1;
            }

            // Allocate the block and return the allocated memory
            let offset = self.free_lists[j].pop().unwrap();
            let memory = GpuAllocatedMemory::new(
                self.memory.clone(),
                memory_type_index,
                block_index as _,
                chunk_index,
                offset,
            );

            if j == self.largest_free_block.unwrap() && self.free_lists[j].len() == 0 {
                while j > 0 {
                    j -= 1;
                    if self.free_lists[j].len() > 0 {
                        self.largest_free_block = Some(j);
                        return Ok(memory);
                    }
                }

                self.largest_free_block = None;
            }

            return Ok(memory);
        }

        panic!("no block found that can fit the requested block size");
    }
}
