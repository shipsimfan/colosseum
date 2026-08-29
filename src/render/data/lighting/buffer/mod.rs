use alexandria::gpu::{VulkanBuffer, VulkanMappedMemory};

mod get;
mod new;
mod push;
mod reset;

/// A contiguous buffer that holds a set number of lighting data elements
pub(in crate::render::data::lighting) struct LightingDataBuffer<T> {
    /// The capacity of the buffer
    capacity: usize,

    /// The current number of objects in the buffer
    count: usize,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanMappedMemory<T>,
}
