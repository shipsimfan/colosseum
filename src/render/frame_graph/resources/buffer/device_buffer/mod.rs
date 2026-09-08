use alexandria::gpu::{
    VulkanBuffer, VulkanBufferUsageFlags, VulkanDescriptorType, VulkanDeviceMemory,
};
use std::ffi::CString;

mod descriptor_set;

mod copy;
mod new;

pub(in crate::render) use descriptor_set::*;

/// A contiguous buffer that holds a set number of elements
pub(in crate::render) struct DeviceDataBuffer {
    /// The name of the data buffer
    name: CString,

    /// The current capacity of the buffer, in bytes
    capacity: usize,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The memory the buffer uses
    #[allow(unused)]
    memory: VulkanDeviceMemory,

    /// The usage of the buffer
    usage: VulkanBufferUsageFlags,

    /// The descriptor sets this buffer should be bound to
    descriptor_sets: Vec<DeviceBufferDescriptorSet>,

    /// The type of descriptor this buffer is used as
    descriptor_type: VulkanDescriptorType,
}
