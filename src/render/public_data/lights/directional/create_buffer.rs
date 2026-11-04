use crate::{
    Result,
    render::{DirectionalLight, GpuDataBuffers, GpuDirectionalLight, GpuDirectionalLightMatrix},
};
use colosseum_ecs::World;

const INITIAL_CAPACITY: usize = 1;

impl DirectionalLight {
    /// Create the GPU data buffer for directional lights
    pub(in crate::render) fn create_buffer(
        data_buffers: &mut GpuDataBuffers,
        world: &mut World,
    ) -> Result<()> {
        data_buffers.create_uniform::<GpuDirectionalLight>(
            "Directional Light Buffer",
            INITIAL_CAPACITY,
            world,
        )?;
        data_buffers.create_uniform::<GpuDirectionalLightMatrix>(
            "Directional Light Matrix Buffer",
            INITIAL_CAPACITY,
            world,
        )
    }
}
