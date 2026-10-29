use alexandria::gpu::{VulkanBuffer, VulkanMappedMemory};

mod new;

/// A raw Vulkan buffer that holds a set number of elements in host-visible
pub(in crate::render::objects::gpu_data_buffer) struct HostDataBuffer<T> {
    /// The name of the data buffer
    buffer_name: String,

    /// The name of the memory associated with the buffer
    memory_name: String,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanMappedMemory<T>,

    /// The size between individual elements in the buffer
    stride: usize,
}
