use alexandria::gpu::{VulkanBuffer, VulkanMappedMemory};

mod allocate_memory;
mod get;
mod index;
mod new;
mod push;
mod reset;

/// The per-frame data for the host-visible data buffer
pub(in crate::render) struct FrameHostDataBuffer<T> {
    /// The name of the data buffer
    buffer_name: String,

    /// The name of the memory associated with the buffer
    memory_name: String,

    /// The capacity of the buffer
    capacity: usize,

    /// The current number of objects in the buffer
    count: usize,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanMappedMemory<T>,

    /// The size between individual elements in the buffer
    stride: usize,
}
