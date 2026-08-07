use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use std::sync::Arc;

impl<'a, T> StagingBuffer<T> {
    /// Create a new [`StagingBuffer`] with the specified capacity
    pub fn new(
        initial_capacity: usize,
        device: VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<StagingBuffer<T>> {
        let (buffer, memory) =
            StagingBuffer::allocate(&device, &memory_properties, initial_capacity)?;

        Ok(StagingBuffer {
            buffer,
            memory,
            capacity: initial_capacity,
            device,
            memory_properties: memory_properties.clone(),
        })
    }
}
