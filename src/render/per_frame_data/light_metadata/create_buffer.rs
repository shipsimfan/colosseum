use crate::{
    Result,
    render::{GpuDataBuffers, LightMetadata},
};
use colosseum_ecs::World;

impl LightMetadata {
    /// Create the GPU data buffer for the light metadata
    pub fn create_buffer(data_buffers: &mut GpuDataBuffers, world: &mut World) -> Result<()> {
        data_buffers.create_uniform_with_data("Light Metadata", &[LightMetadata::default()], world)
    }
}
