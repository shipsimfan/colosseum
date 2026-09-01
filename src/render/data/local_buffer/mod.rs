use alexandria::gpu::{VulkanBuffer, VulkanMappedMemory};

mod get;
mod index;
mod new;
mod push;
mod reserve;
mod reset;

/// A contiguous buffer that holds a set number of elements
pub(in crate::render) struct LocalDataBuffer<T> {
    /// The capacity of the buffer
    capacity: usize,

    /// The current number of objects in the buffer
    count: usize,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanMappedMemory<T>,
}
