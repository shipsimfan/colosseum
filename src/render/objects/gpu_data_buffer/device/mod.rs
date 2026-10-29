use alexandria::gpu::{VulkanBuffer, VulkanDeviceMemory};
use std::marker::PhantomData;

mod new;

/// A raw Vulkan buffer that holds a set number of elements in device-local memory
pub(in crate::render::objects::gpu_data_buffer) struct DeviceDataBuffer<T> {
    /// The name of the data buffer
    buffer_name: String,

    /// The name of the memory associated with the buffer
    memory_name: String,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanDeviceMemory,

    /// A marker for the contained type
    _marker: PhantomData<T>,
}
