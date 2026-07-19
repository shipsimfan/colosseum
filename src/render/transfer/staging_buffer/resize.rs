use crate::{Error, Result, render::transfer::StagingBuffer};
use alexandria::gpu::{
    VulkanBuffer, VulkanBufferUsageFlag, VulkanDevice, VulkanMappedMemory, VulkanSharingMode,
};

impl<T> StagingBuffer<T> {
    /// Resize the staging buffer to the specified capacity
    pub(in crate::render::transfer::staging_buffer) fn resize(
        &mut self,
        new_capacity: usize,
    ) -> Result<()> {
        let (buffer, memory) =
            StagingBuffer::allocate(&self.device, self.memory_type, new_capacity)?;

        self.buffer = buffer;
        self.memory = memory;
        self.capacity = new_capacity;

        Ok(())
    }

    /// Allocate a new staging buffer with the specified capacity
    pub(in crate::render::transfer::staging_buffer) fn allocate(
        device: &VulkanDevice,
        memory_type: usize,
        capacity: usize,
    ) -> Result<(VulkanBuffer, VulkanMappedMemory<T>)> {
        let size = capacity as u64 * std::mem::size_of::<T>() as u64;

        // Create the buffer
        let mut buffer = device
            .create_buffer(
                0,
                size,
                VulkanBufferUsageFlag::TransferSrc,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(Error::new_inner)?;

        // Allocate the memory for the buffer
        let memory_requirements = buffer.get_memory_requirements();
        let memory = device
            .allocate_memory(memory_requirements.size(), memory_type)
            .map_err(Error::new_inner)?;

        // Bind the buffer and memory
        buffer.bind_memory(&memory, 0).unwrap();

        // Map the memory
        let memory = memory
            .map(0, size, 0)
            .map_err(|(error, _)| Error::new_inner(error))?;

        Ok((buffer, memory))
    }
}
