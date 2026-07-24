use crate::{
    Result,
    update::render_objects::{GpuAllocatedMemory, GpuAllocator},
};
use alexandria::gpu::VulkanMemoryRequirements;

impl GpuAllocator {
    /// Allocate a block of GPU memory for the given memory requirements
    pub(in crate::update::render_objects) fn allocate(
        &mut self,
        memory_requirements: &VulkanMemoryRequirements,
    ) -> Result<GpuAllocatedMemory> {
        // Check if we need to have a dedicated allocation
        let size = *memory_requirements.size() as u32;
        if size > self.max_block_size {
            return self.dedicated_allocation(memory_requirements);
        }

        // Determine the block index based on the size of the allocation
        let size_log2 = size
            .max(self.min_block_size)
            .next_power_of_two()
            .trailing_zeros();
        let min_block_size_log2 = self.min_block_size.trailing_zeros();
        let block_index = (size_log2 - min_block_size_log2) as usize;

        // Check if a memory type supports the given memory requirements and allocate from it
        for memory_type in &mut self.memory_types {
            if !memory_type.supports(memory_requirements) {
                continue;
            }

            return memory_type.allocate(block_index);
        }

        // If no memory type supports the given memory requirements, try to create a new one
        todo!()
    }

    /// Allocate a dedicated block of GPU memory for the given memory requirements
    fn dedicated_allocation(
        &self,
        memory_requirements: &VulkanMemoryRequirements,
    ) -> Result<GpuAllocatedMemory> {
        todo!()
    }
}
