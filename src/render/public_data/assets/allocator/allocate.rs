use crate::{
    Result,
    render::{
        GpuAllocatedMemory,
        public_data::assets::{GpuAllocator, allocator::GpuMemoryType},
    },
};
use alexandria::gpu::{VulkanMemoryPropertyFlag, VulkanMemoryRequirements};
use colosseum_core::new_error;
use std::sync::Arc;

impl GpuAllocator {
    /// Allocate a block of GPU memory for the given memory requirements
    pub(in crate::render::public_data::assets) fn allocate(
        &mut self,
        memory_requirements: &VulkanMemoryRequirements,
        name: &str,
    ) -> Result<GpuAllocatedMemory> {
        // Check if we need to have a dedicated allocation
        let size = *memory_requirements.size() as u32;
        if size > self.max_block_size {
            return self.dedicated_allocation(memory_requirements, name);
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
                return memory_type.allocate(block_index, self.name);
            }
        }

        // If no memory type supports the given memory requirements, try to create a new one
        let memory_type_index = self
            .device
            .memory_properties()
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::DeviceLocal,
            )
            .ok_or_else(|| new_error!("unable to find a suitable memory type for a buffer"))?;
        self.memory_types.push(GpuMemoryType::new(
            memory_type_index as _,
            self.chunk_size,
            self.min_block_size,
            self.max_block_size,
            self.memory_types.len() as _,
            (*self.device).clone(),
        ));

        self.memory_types
            .last_mut()
            .unwrap()
            .allocate(block_index, self.name)
    }

    /// Allocate a dedicated block of GPU memory for the given memory requirements
    fn dedicated_allocation(
        &self,
        memory_requirements: &VulkanMemoryRequirements,
        name: &str,
    ) -> Result<GpuAllocatedMemory> {
        let device_memory = self.device.allocate_memory(
            memory_requirements,
            VulkanMemoryPropertyFlag::DeviceLocal,
            &format!("\"{}\" Memory", name),
        )?;

        Ok(GpuAllocatedMemory::new_dedicated(Arc::new(device_memory)))
    }
}
