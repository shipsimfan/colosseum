use alexandria::gpu::{GpuAddress, VulkanBuffer, VulkanMappedMemory};

mod get;
mod new;
mod push;
mod reset;

/// A contiguous buffer that holds a set number of elements
pub(in crate::render::data::doubled) struct DataBuffer<T> {
    /// The capacity of the buffer
    capacity: usize,

    /// The current number of objects in the buffer
    count: usize,

    /// The GPU buffer containing the object data
    #[allow(unused)]
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanMappedMemory<T>,

    /// The base address of the buffer in GPU memory
    base_address: GpuAddress<T>,
}
