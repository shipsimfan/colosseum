use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::VulkanBuffer;

impl<'a, T> StagingBuffer<'a, T> {
    /// Set the contents of the staging buffer to the specified data
    pub fn set(&mut self, data: &[T]) -> Result<&VulkanBuffer> {
        if data.len() > self.capacity {
            self.resize(self.capacity * 2)?;
        }

        // Copy the data into the mapped memory
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), self.memory.as_mut_ptr(), data.len());
        }

        Ok(&self.buffer)
    }
}
