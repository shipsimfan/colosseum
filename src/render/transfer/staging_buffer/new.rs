use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl<'a, T> StagingBuffer<'a, T> {
    /// Create a new [`StagingBuffer`] with the specified capacity
    pub fn new(
        initial_capacity: usize,
        device: VulkanDevice,
        memory_properties: &'a VulkanAdapterMemoryProperties,
    ) -> Result<StagingBuffer<'a, T>> {
        let (buffer, memory) =
            StagingBuffer::allocate(&device, memory_properties, initial_capacity)?;

        Ok(StagingBuffer {
            buffer,
            memory,
            capacity: initial_capacity,
            device,
            memory_properties,
        })
    }
}
