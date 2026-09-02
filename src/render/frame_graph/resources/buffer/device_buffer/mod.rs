use alexandria::gpu::{
    VulkanBuffer, VulkanBufferUsageFlags, VulkanDescriptorType, VulkanDeviceMemory,
};

mod copy;
mod new;

/// A contiguous buffer that holds a set number of elements
pub(in crate::render) struct DeviceDataBuffer {
    /// The current capacity of the buffer, in bytes
    capacity: usize,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The memory the buffer uses
    #[allow(unused)]
    memory: VulkanDeviceMemory,

    /// The usage of the buffer
    usage: VulkanBufferUsageFlags,

    /// The binding that this device buffer is bound to in the descriptor set
    binding: u32,

    /// The type of descriptor this buffer is used as
    descriptor_type: VulkanDescriptorType,
}
