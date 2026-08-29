use crate::render::data::lighting::LightingDataBuffer;
use alexandria::gpu::VulkanBuffer;

impl<T> LightingDataBuffer<T> {
    /// Get the capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the [`VulkanBuffer`] that contains the data
    pub fn buffer(&self) -> &VulkanBuffer {
        &self.buffer
    }
}
