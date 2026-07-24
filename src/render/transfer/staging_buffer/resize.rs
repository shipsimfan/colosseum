use crate::{Error, Result, render::transfer::StagingBuffer};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBuffer, VulkanBufferUsageFlag, VulkanDevice,
    VulkanMappedMemory, VulkanMemoryPropertyFlag, VulkanSharingMode,
};

impl<'a, T> StagingBuffer<'a, T> {
    /// Resize the staging buffer to the specified capacity
    pub(in crate::render::transfer::staging_buffer) fn resize(
        &mut self,
        new_capacity: usize,
    ) -> Result<()> {
        let (buffer, memory) =
            StagingBuffer::allocate(&self.device, self.memory_properties, new_capacity)?;

        self.buffer = buffer;
        self.memory = memory;
        self.capacity = new_capacity;

        Ok(())
    }

    /// Allocate a new staging buffer with the specified capacity
    pub(in crate::render::transfer::staging_buffer) fn allocate(
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
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
        let memory_type =
            find_memory_type(memory_properties, memory_requirements.memory_type_bits())?;
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

fn find_memory_type(
    memory_properties: &VulkanAdapterMemoryProperties,
    type_filter: u32,
) -> Result<usize> {
    for (i, memory_type) in memory_properties.memory_types().iter().enumerate() {
        if (type_filter & (1 << i)) != 0
            && memory_type.flags().contains(
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
        {
            return Ok(i);
        }
    }

    Err(Error::new(
        "unable to find a suitable memory type for a staging buffer",
    ))
}
