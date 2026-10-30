use crate::render::FrameHostDataBuffer;
use alexandria::gpu::VulkanBuffer;

impl<T> FrameHostDataBuffer<T> {
    /// Get the current count of objects in the buffer
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get the current capacity of the buffer
    pub(in crate::render::per_frame_data::data_buffers) fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the size of the buffer in bytes
    pub(in crate::render::per_frame_data::data_buffers) fn size(&self) -> usize {
        self.count * self.stride
    }

    /// Get a reference to the buffer
    pub(in crate::render::per_frame_data::data_buffers) fn buffer(&self) -> &VulkanBuffer {
        &self.buffer
    }
}
