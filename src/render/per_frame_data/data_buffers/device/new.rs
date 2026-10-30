use crate::{
    Result,
    render::{
        GraphicsDevice,
        per_frame_data::data_buffers::{DeviceDataBuffer, FrameDeviceDataBuffer},
    },
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanBufferUsageFlags};

impl<T> DeviceDataBuffer<T> {
    /// Create a new [`DeviceDataBuffer`]
    pub(in crate::render::per_frame_data::data_buffers) fn new(
        name: &str,
        capacity: usize,
        mut usage: VulkanBufferUsageFlags,
        host_index: usize,
        device: &GraphicsDevice,
    ) -> Result<DeviceDataBuffer<T>> {
        usage |= VulkanBufferUsageFlag::TransferDst;

        Ok(DeviceDataBuffer {
            frame: [
                FrameDeviceDataBuffer::new(name, 0, capacity, usage, device)?,
                FrameDeviceDataBuffer::new(name, 1, capacity, usage, device)?,
                FrameDeviceDataBuffer::new(name, 2, capacity, usage, device)?,
            ],
            host_index,
        })
    }
}
