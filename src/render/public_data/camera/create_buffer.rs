use crate::{
    Result,
    render::{Camera, GpuCamera, GpuDataBuffer, GraphicsDevice},
};
use alexandria::gpu::VulkanBufferUsageFlag;

impl Camera {
    /// Create the GPU data buffer for the camera
    pub(in crate::render::public_data) fn create_buffer(
        device: &GraphicsDevice,
    ) -> Result<GpuDataBuffer<GpuCamera>> {
        GpuDataBuffer::new(
            "Camera Buffer",
            1,
            VulkanBufferUsageFlag::UniformBuffer,
            device,
        )
    }
}
