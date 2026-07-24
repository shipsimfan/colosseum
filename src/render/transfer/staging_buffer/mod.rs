use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBuffer, VulkanDevice, VulkanMappedMemory,
};

mod new;
mod resize;
mod set;

/// A staging buffer for temporarily holding data before transferring it to the GPU
pub(in crate::render::transfer) struct StagingBuffer<'a, T> {
    /// The Vulkan buffer used for staging
    buffer: VulkanBuffer,

    /// The mapped memory of the staging buffer
    memory: VulkanMappedMemory<T>,

    /// The capacity of the staging buffer in elements
    capacity: usize,

    /// The device to use when resizing the staging buffer
    device: VulkanDevice,

    /// The memory properties of the adapter to use when resizing the staging buffer
    memory_properties: &'a VulkanAdapterMemoryProperties,
}
