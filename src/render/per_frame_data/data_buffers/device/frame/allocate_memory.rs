use crate::{
    Result,
    render::{GraphicsDevice, per_frame_data::data_buffers::FrameDeviceDataBuffer},
};
use alexandria::gpu::{
    VulkanBuffer, VulkanBufferUsageFlags, VulkanDeviceMemory, VulkanMemoryPropertyFlag,
    VulkanSharingMode,
};
use colosseum_core::new_error;
use std::alloc::Layout;
#[cfg(debug_assertions)]
use std::{ffi::CString, str::FromStr};

impl<T> FrameDeviceDataBuffer<T> {
    /// Allocate a Vulkan buffer and its associated host-visible memory
    pub(in crate::render::per_frame_data::data_buffers::device::frame) fn allocate_memory(
        buffer_name: &str,
        memory_name: &str,
        capacity: usize,
        usage: VulkanBufferUsageFlags,
        device: &GraphicsDevice,
    ) -> Result<(VulkanBuffer, VulkanDeviceMemory)> {
        let size = Layout::new::<T>().repeat(capacity).unwrap().0.size() as u64;

        // Create the buffer
        let mut buffer = device
            .create_buffer(0, size, usage, VulkanSharingMode::Exclusive, &[])
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
