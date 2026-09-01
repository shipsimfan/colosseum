use alexandria::gpu::{VulkanBuffer, VulkanBufferUsageFlags, VulkanDeviceMemory};

mod copy;
mod new;

/// A contiguous buffer that holds a set number of elements
pub(in crate::render) struct DeviceDataBuffer {
    /// The current capacity of the buffer, in bytes
    capacity: usize,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanDeviceMemory,

    /// The usage of the buffer
    usage: VulkanBufferUsageFlags,
}
