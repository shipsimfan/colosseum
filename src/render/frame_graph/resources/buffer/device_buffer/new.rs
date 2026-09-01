use crate::{Error, Result, render::DeviceDataBuffer};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferUsageFlag, VulkanBufferUsageFlags, VulkanDevice,
    VulkanMemoryPropertyFlag, VulkanSharingMode,
};

impl DeviceDataBuffer {
    /// Create a new [`DeviceDataBuffer`]
    pub fn new<T, U: Into<VulkanBufferUsageFlags>>(
        capacity: usize,
        usage: U,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<DeviceDataBuffer> {
        let size = (capacity * std::mem::size_of::<T>()) as u64;

        // Create the buffer
        let usage = VulkanBufferUsageFlag::TransferDst | usage.into();
        let mut buffer = device
            .create_buffer(0, size, usage, VulkanSharingMode::Exclusive, &[])
            .map_err(Error::new_inner)?;

        // Allocate memory for the buffer
        let memory_requirements = buffer.get_memory_requirements();
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::DeviceLocal,
            )
            .ok_or(Error::new("cannot find memory for a device buffer"))?;
        let memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;

        // Bind the buffer to the allocated memory
        buffer.bind_memory(&memory, 0).map_err(Error::new_inner)?;

        Ok(DeviceDataBuffer {
            capacity: size as usize,
            buffer,
            memory,
            usage,
        })
    }
}
