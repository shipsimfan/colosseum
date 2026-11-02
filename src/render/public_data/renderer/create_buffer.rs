use crate::{
    Result,
    render::{GpuDataBuffers, GpuRenderable, Renderer},
};
use colosseum_ecs::World;

impl Renderer {
    /// Create the GPU data buffer for the camera
    pub(in crate::render) fn create_buffer(
        data_buffers: &mut GpuDataBuffers,
        world: &mut World,
    ) -> Result<()> {
        data_buffers.create_storage::<GpuRenderable>("Renderable Buffer", 256, world)
    }
}
