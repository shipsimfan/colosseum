use crate::{
    Result,
    render::{GraphicsDevice, per_frame_data::data_buffers::FrameDeviceDataBuffer},
};
use alexandria::gpu::VulkanBufferUsageFlags;
use std::marker::PhantomData;

impl<T> FrameDeviceDataBuffer<T> {
    /// Create a new [`FrameDeviceDataBuffer`]
    pub(in crate::render::per_frame_data::data_buffers::device) fn new(
        name: &str,
        index: usize,
        capacity: usize,
        usage: VulkanBufferUsageFlags,
        device: &GraphicsDevice,
    ) -> Result<FrameDeviceDataBuffer<T>> {
        let buffer_name = format!("Device {} {}", name, index);
        let memory_name = format!("Device {} Memory {}", name, index);

        let (buffer, memory) = FrameDeviceDataBuffer::<T>::allocate_memory(
            &buffer_name,
            &memory_name,
            capacity,
            usage,
            device,
        )?;

        Ok(FrameDeviceDataBuffer {
            buffer_name,
            memory_name,
            capacity,
            usage,
            buffer,
            memory,
            _marker: PhantomData,
        })
    }
}
