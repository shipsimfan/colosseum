use crate::{
    Result,
    render::{GraphicsDevice, objects::gpu_data_buffer::DeviceDataBuffer},
};
use alexandria::gpu::{
    VulkanBuffer, VulkanBufferUsageFlag, VulkanBufferUsageFlags, VulkanDeviceMemory,
    VulkanMemoryPropertyFlag, VulkanSharingMode,
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::str::FromStr;
use std::{alloc::Layout, ffi::CString, marker::PhantomData};

impl<T> DeviceDataBuffer<T> {
    /// Create a new [`DeviceDataBuffer`]
    pub fn new(
        name: &str,
        index: usize,
        capacity: usize,
        usage: VulkanBufferUsageFlags,
        device: &GraphicsDevice,
    ) -> Result<DeviceDataBuffer<T>> {
        let buffer_name = format!("Device {} {}", name, index);
        let memory_name = format!("Device {} Memory {}", name, index);

        let (buffer, memory) = DeviceDataBuffer::<T>::allocate_memory(
            &buffer_name,
            &memory_name,
            capacity,
            usage,
            device,
        )?;

        Ok(DeviceDataBuffer {
            buffer_name,
            memory_name,
            buffer,
            memory,
            _marker: PhantomData,
        })
    }

    /// Allocate a Vulkan buffer and its associated host-visible memory
    fn allocate_memory(
        buffer_name: &str,
        memory_name: &str,
        capacity: usize,
        usage: VulkanBufferUsageFlags,
        device: &GraphicsDevice,
    ) -> Result<(VulkanBuffer, VulkanDeviceMemory)> {
        let size = Layout::new::<T>().repeat(capacity).unwrap().0.size() as u64;

        // Create the buffer
        let mut buffer = device
            .create_buffer(
                0,
                size,
                usage | VulkanBufferUsageFlag::TransferDst,
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
            VulkanMemoryPropertyFlag::DeviceLocal,
            memory_name,
        )?;

        // Bind the buffer to the allocated memory
        buffer.bind_memory(&memory, 0).map_err(|error| {
            new_error!("unable to bind memory for \"{}\" - {}", buffer_name, error)
        })?;

        Ok((buffer, memory))
    }
}
