use crate::render::GpuAllocatedMemory;

impl GpuAllocatedMemory {
    /// Get the index of the memory type that this allocated memory is using
    ///
    /// If this is [`None`], then this is a dedicated memory allocation
    pub(in crate::render::public_data::assets::allocator) fn memory_type_index(
        &self,
    ) -> Option<u8> {
        self.memory_type_index
    }

    /// Get the index of the free list to insert this block into
    pub(in crate::render::public_data::assets::allocator) fn size(&self) -> u8 {
        self.size
    }

    /// Get the index of the chunk that this allocated memory is using
    pub(in crate::render::public_data::assets::allocator) fn chunk_index(&self) -> u16 {
        self.chunk_index
    }

    /// Get the offset into the device memory that this allocated memory is using
    pub(in crate::render::public_data::assets::allocator) fn offset(&self) -> u32 {
        self.offset
    }
}
