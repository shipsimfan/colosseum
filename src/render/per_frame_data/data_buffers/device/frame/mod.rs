use alexandria::gpu::{VulkanBuffer, VulkanBufferUsageFlags, VulkanDeviceMemory};
use std::marker::PhantomData;

mod allocate_memory;
mod get;
mod new;
mod resize;

/// The per-frame data for the device-local data buffer
pub(in crate::render::per_frame_data::data_buffers) struct FrameDeviceDataBuffer<T> {
    /// The name of the data buffer
    buffer_name: String,

    /// The name of the memory associated with the buffer
    memory_name: String,

    /// The capacity of the buffer
    capacity: usize,

    /// The usage flags for the buffer
    usage: VulkanBufferUsageFlags,

    /// The GPU buffer containing the object data
    buffer: VulkanBuffer,

    /// The mapped memory for writing object data
    memory: VulkanDeviceMemory,

    /// A marker for the contained type
    _marker: PhantomData<T>,
}
