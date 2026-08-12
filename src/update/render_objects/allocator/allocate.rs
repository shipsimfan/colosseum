use crate::{
    Error, Result,
    update::render_objects::{GpuAllocatedMemory, GpuAllocator, allocator::GpuMemoryType},
};
use alexandria::gpu::{VulkanMemoryPropertyFlag, VulkanMemoryRequirements};
use std::sync::Arc;

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
            if memory_type.supports(memory_requirements) {
                return memory_type.allocate(block_index);
            }
        }

        // If no memory type supports the given memory requirements, try to create a new one
        let memory_type_index = self
            .memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
            .ok_or(Error::new(
                "unable to find a suitable memory type for a buffer",
            ))?;
        self.memory_types.push(GpuMemoryType::new(
            memory_type_index as _,
            self.chunk_size,
            self.min_block_size,
            self.max_block_size,
            self.memory_types.len() as _,
            self.device.clone(),
        ));

        self.memory_types.last_mut().unwrap().allocate(block_index)
    }

    /// Allocate a dedicated block of GPU memory for the given memory requirements
    fn dedicated_allocation(
        &self,
        memory_requirements: &VulkanMemoryRequirements,
    ) -> Result<GpuAllocatedMemory> {
        let memory_type_index = self
            .memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
            .ok_or(Error::new(
                "unable to find a suitable memory type for a buffer",
            ))?;

        let device_memory = Arc::new(
            self.device
                .allocate_memory(memory_requirements.size(), memory_type_index as _)
                .map_err(Error::new_inner)?,
        );

        Ok(GpuAllocatedMemory::new_dedicated(device_memory))
    }
}
