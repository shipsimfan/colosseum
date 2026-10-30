use crate::{
    Result,
    render::{Camera, GpuCamera, GpuDataBuffers},
};
use colosseum_ecs::World;

impl Camera {
    /// Create the GPU data buffer for the camera
    pub(in crate::render) fn create_buffer(
        data_buffers: &mut GpuDataBuffers,
        world: &mut World,
    ) -> Result<()> {
        data_buffers.create_uniform_with_data::<GpuCamera>(
            "Camera Buffer",
            &[GpuCamera::default()],
            world,
        )
    }
}
