use crate::{
    Result,
    render::{
        GpuDataBuffer, GraphicsDevice,
        objects::gpu_data_buffer::{DeviceDataBuffer, GpuDataBufferInner, HostDataBuffer},
    },
};
use alexandria::gpu::VulkanBufferUsageFlags;

impl<T> GpuDataBuffer<T> {
    /// Create a new [`DeviceDataBuffer`]
    pub fn new<S: Into<String>, F: Into<VulkanBufferUsageFlags>>(
        name: S,
        capacity: usize,
        usage: F,
        device: &GraphicsDevice,
    ) -> Result<GpuDataBuffer<T>> {
        let usage = usage.into();
        let name = name.into();

        Ok(GpuDataBuffer {
            inner: [
                GpuDataBufferInner::new(&name, 0, capacity, usage, device)?,
                GpuDataBufferInner::new(&name, 1, capacity, usage, device)?,
                GpuDataBufferInner::new(&name, 2, capacity, usage, device)?,
            ],
            name,
        })
    }
}

impl<T> GpuDataBufferInner<T> {
    /// Create a new [`GpuDataBufferInner`]
    fn new(
        name: &str,
        index: usize,
        capacity: usize,
        usage: VulkanBufferUsageFlags,
        device: &GraphicsDevice,
    ) -> Result<GpuDataBufferInner<T>> {
        let host_buffer = HostDataBuffer::new(name, index, capacity, device)?;
        let device_buffer = DeviceDataBuffer::new(name, index, capacity, usage, device)?;

        Ok(GpuDataBufferInner {
            capacity,
            count: 0,
            host_buffer,
            device_buffer,
        })
    }
}
