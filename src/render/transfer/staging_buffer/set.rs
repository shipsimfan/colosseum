use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::VulkanBuffer;

impl StagingBuffer {
    /// Set the contents of the staging buffer to the specified data
    pub fn set<T>(&mut self, data: &[T], offset: usize) -> Result<&VulkanBuffer> {
        let size = data.len() * std::mem::size_of::<T>();

        assert!(offset + size <= self.memory.len());

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr().cast(),
                self.memory.as_mut_ptr().add(offset),
                size,
            );
        }

        Ok(&self.buffer)
    }
}
