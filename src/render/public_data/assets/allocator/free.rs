use crate::render::{GpuAllocatedMemory, public_data::assets::GpuAllocator};

impl GpuAllocator {
    /// Free a block of GPU memory that was previously allocated
    pub(in crate::render::public_data::assets) fn free(&mut self, memory: GpuAllocatedMemory) {
        if let Some(memory_type_index) = memory.memory_type_index() {
            self.memory_types[memory_type_index as usize].free(memory);
        }
    }
}
