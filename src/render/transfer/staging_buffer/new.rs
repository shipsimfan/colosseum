use crate::{Result, render::transfer::StagingBuffer};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
use std::{ffi::CString, sync::Arc};

impl StagingBuffer {
    /// Create a new [`StagingBuffer`] with the specified capacity
    pub fn new(
        name: &str,
        initial_capacity: usize,
        device: VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<StagingBuffer> {
        let memory_name = CString::new(format!("{} Memory", name)).unwrap();
        let buffer_name = CString::new(name).unwrap();

        let (buffer, memory) = StagingBuffer::allocate(
            &buffer_name,
            &memory_name,
            &device,
            &memory_properties,
            initial_capacity,
        )?;

        Ok(StagingBuffer {
            buffer_name,
            memory_name,
            buffer,
            memory,
            device,
            memory_properties: memory_properties.clone(),
        })
    }
}
