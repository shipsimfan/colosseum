use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBuffer, VulkanBufferUsageFlag, VulkanDevice,
    VulkanMappedMemory, VulkanMemoryPropertyFlag, VulkanSharingMode,
};
use colosseum_core::new_error;
use std::ffi::CStr;

impl StagingBuffer {
    /// Resize the staging buffer to the specified capacity
    pub fn resize(&mut self, new_size: usize) -> Result<()> {
        if self.memory.len() >= new_size {
            return Ok(());
        }

        let (buffer, memory) = StagingBuffer::allocate(
            &self.buffer_name,
            &self.memory_name,
            &self.device,
            &self.memory_properties,
            new_size,
        )?;

        self.buffer = buffer;
        self.memory = memory;

        Ok(())
    }

    /// Allocate a new staging buffer with the specified capacity
    pub(in crate::render::transfer::staging_buffer) fn allocate(
        #[cfg_attr(not(debug_assertions), allow(unused_variables))] buffer_name: &CStr,
        #[cfg_attr(not(debug_assertions), allow(unused_variables))] memory_name: &CStr,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
        size: usize,
    ) -> Result<(VulkanBuffer, VulkanMappedMemory<u8>)> {
        // Create the buffer
        let mut buffer = device
            .create_buffer(
                0,
                size as _,
                VulkanBufferUsageFlag::TransferSrc,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(|error| new_error!("unable to create staging buffer - {}", error))?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut buffer, buffer_name)
            .map_err(|error| {
                new_error!("unable to set object name for staging buffer - {}", error)
            })?;

        // Allocate the memory for the buffer
        let memory_requirements = buffer.get_memory_requirements();
        let memory_type = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
            .ok_or_else(|| {
                new_error!("unable to find a suitable memory type for a staging buffer")
            })?;
        let mut memory = device
            .allocate_memory(memory_requirements.size(), memory_type)
            .map_err(|error| {
                new_error!("unable to allocate memory for staging buffer - {}", error)
            })?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut memory, memory_name)
            .map_err(|error| {
                new_error!(
                    "unable to set object name for staging buffer memory - {}",
                    error
                )
            })?;

        // Bind the buffer and memory
        buffer.bind_memory(&memory, 0).unwrap();

        // Map the memory
        let memory = memory
            .map(0, size, 0)
            .map_err(|(error, _)| new_error!("unable to map staging buffer memory - {}", error))?;

        Ok((buffer, memory))
    }
}
