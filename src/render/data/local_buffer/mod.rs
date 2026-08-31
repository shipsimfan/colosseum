use alexandria::gpu::{VulkanBuffer, VulkanMappedMemory};

mod index;
mod new;
mod push;
mod reserve;
mod reset;

/// A contiguous buffer that holds a set number of elements
pub(in crate::render::data) struct LocalDataBuffer<T> {
    /// The capacity of the buffer
    capacity: usize,

    /// The current number of objects in the buffer
    count: usize,

    /// The GPU buffer containing the object data
    #[allow(unused)]
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanMappedMemory<T>,
}
