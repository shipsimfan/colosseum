use crate::{
    Result,
    render::{GraphicsDevice, objects::gpu_data_buffer::HostDataBuffer},
};
use alexandria::gpu::{
    VulkanBuffer, VulkanBufferUsageFlag, VulkanMappedMemory, VulkanMemoryPropertyFlag,
    VulkanSharingMode,
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::str::FromStr;
use std::{alloc::Layout, ffi::CString};

impl<T> HostDataBuffer<T> {
    /// Create a new [`HostDataBuffer`]
    pub fn new(
        name: &str,
        index: usize,
        capacity: usize,
        device: &GraphicsDevice,
    ) -> Result<HostDataBuffer<T>> {
        let buffer_name = format!("Host {} {}", name, index);
        let memory_name = format!("Host {} Memory {}", name, index);

        let (buffer, memory, stride) =
            HostDataBuffer::allocate_memory(&buffer_name, &memory_name, capacity, device)?;

        Ok(HostDataBuffer {
            buffer_name,
            memory_name,
            buffer,
            memory,
            stride,
        })
    }

    /// Allocate a Vulkan buffer and its associated host-visible memory
    fn allocate_memory(
        buffer_name: &str,
        memory_name: &str,
        capacity: usize,
        device: &GraphicsDevice,
    ) -> Result<(VulkanBuffer, VulkanMappedMemory<T>, usize)> {
        let (layout, stride) = Layout::new::<T>().repeat(capacity).unwrap();
        let size = layout.size() as u64;

        // Create the buffer
        let mut buffer = device
            .create_buffer(
                0,
                size,
                VulkanBufferUsageFlag::TransferSrc,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(|error| new_error!("unable to create \"{}\" - {}", buffer_name, error))?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut buffer, &CString::from_str(buffer_name).unwrap())
            .map_err(|error| {
                new_error!(
                    "unable to set object name for \"{}\" - {}",
                    buffer_name,
                    error
                )
            })?;

        // Allocate memory for the buffer
        let memory = device.allocate_memory(
            &buffer.get_memory_requirements(),
            VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            memory_name,
        )?;

        // Bind the buffer to the allocated memory
        buffer.bind_memory(&memory, 0).map_err(|error| {
            new_error!("unable to bind memory for \"{}\" - {}", buffer_name, error)
        })?;

        // Map the buffer memory to a pointer
        let memory = memory.map(0, size, 0).map_err(|(error, _)| {
            new_error!("unable to map memory for \"{}\" - {}", buffer_name, error)
        })?;

        Ok((buffer, memory, stride))
    }
}
