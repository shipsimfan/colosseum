use crate::update::{GpuAllocatedMemory, render_objects::allocator::GpuMemoryChunk};

impl GpuMemoryChunk {
    /// Free a block of GPU memory that was previously allocated
    pub fn free(&mut self, memory: GpuAllocatedMemory) {
        let mut offset = memory.offset();
        let mut free_list = memory.size() as usize;
        let mut block_size = 1 << (free_list as u32 + self.min_block_size_log2);

        // Check for coalescing with buddies up the tree
        while self.free_lists[free_list].len() > 0 && free_list < self.free_lists.len() - 1 {
            // Determine the current buddy's offset
            let (buddy_offset, coalesced_offset) = if (offset / block_size) % 2 == 0 {
                (offset + block_size, offset)
            } else {
                (offset - block_size, offset - block_size)
            };

            // Find if the buddy is free
            let mut buddy = None;
            for (i, &free_offset) in self.free_lists[free_list].iter().enumerate() {
                if free_offset == buddy_offset {
                    buddy = Some(i);
                    break;
                }
            }

            // If the buddy is free, remove it from the free list and coalesce
            if buddy.is_none() {
                break;
            }

            self.free_lists[free_list].swap_remove(buddy.unwrap());
            offset = coalesced_offset;
            free_list += 1;
            block_size *= 2;
        }

        // Insert the final free block into the free list
        self.free_lists[free_list].push(offset);

        // Check if we need to update the largest free block index
        if self.largest_free_block.is_none() || free_list > self.largest_free_block.unwrap() {
            self.largest_free_block = Some(free_list);
        }
    }
}
