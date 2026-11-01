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

    /// Get the epoch, Vulkan buffer, and capacity of the frame device data buffer
    pub fn epoch_and_buffer(&self) -> (u64, &VulkanBuffer, u64) {
        (self.epoch, &self.buffer, self.size)
    }
}
