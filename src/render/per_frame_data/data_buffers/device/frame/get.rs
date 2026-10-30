use crate::render::per_frame_data::data_buffers::FrameDeviceDataBuffer;
use alexandria::gpu::VulkanBuffer;

impl<T> FrameDeviceDataBuffer<T> {
    /// Get the current capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get a reference to the buffer
    pub fn buffer(&self) -> &VulkanBuffer {
        &self.buffer
    }
}
