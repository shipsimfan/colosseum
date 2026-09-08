use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use std::{ffi::CStr, sync::Arc};

impl<'a, T> StagingBuffer<T> {
    /// Create a new [`StagingBuffer`] with the specified capacity
    pub fn new(
        name: &'static CStr,
        initial_capacity: usize,
        device: VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<StagingBuffer<T>> {
        let (buffer, memory) =
            StagingBuffer::allocate(name, &device, &memory_properties, initial_capacity)?;

        Ok(StagingBuffer {
            name,
            buffer,
            memory,
            capacity: initial_capacity,
            device,
            memory_properties: memory_properties.clone(),
        })
    }
}
