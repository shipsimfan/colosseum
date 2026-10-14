use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBuffer, VulkanDevice, VulkanMappedMemory,
};
use std::{ffi::CString, sync::Arc};

mod new;
mod resize;
mod set;

/// A staging buffer for temporarily holding data before transferring it to the GPU
pub(in crate::render::transfer) struct StagingBuffer {
    /// The name of the buffer
    buffer_name: CString,

    /// The name of the memory associated with the staging buffer
    memory_name: CString,

    /// The Vulkan buffer used for staging
    buffer: VulkanBuffer,

    /// The mapped memory of the staging buffer
    memory: VulkanMappedMemory<u8>,

    /// The device to use when resizing the staging buffer
    device: VulkanDevice,

    /// The memory properties of the adapter to use when resizing the staging buffer
    memory_properties: Arc<VulkanAdapterMemoryProperties>,
}

unsafe impl Send for StagingBuffer {}
