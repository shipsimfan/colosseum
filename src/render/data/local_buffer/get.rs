use crate::render::LocalDataBuffer;
use alexandria::gpu::VulkanBuffer;

impl<T> LocalDataBuffer<T> {
    /// Get the current capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the current number of objects in the buffer
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get a reference to the underlying GPU buffer
    pub fn buffer(&self) -> &VulkanBuffer {
        &self.buffer
    }
}
