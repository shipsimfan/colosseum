use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::VulkanDevice;

impl<T> StagingBuffer<T> {
    /// Create a new [`StagingBuffer`] with the specified capacity
    pub fn new(
        initial_capacity: usize,
        device: VulkanDevice,
        memory_type: usize,
    ) -> Result<StagingBuffer<T>> {
        let (buffer, memory) = StagingBuffer::allocate(&device, memory_type, initial_capacity)?;

        Ok(StagingBuffer {
            buffer,
            memory,
            capacity: initial_capacity,
            device,
            memory_type,
        })
    }
}
