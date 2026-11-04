use crate::{
    Result,
    render::{GpuDataBuffers, GpuPointLight, GpuPointLightMatrix, PointLight},
};
use colosseum_ecs::World;

const INITIAL_CAPACITY: usize = 16;

impl PointLight {
    /// Create the GPU data buffer for point lights
    pub(in crate::render) fn create_buffer(
        data_buffers: &mut GpuDataBuffers,
        world: &mut World,
    ) -> Result<()> {
        data_buffers.create_uniform::<GpuPointLight>(
            "Point Light Buffer",
            INITIAL_CAPACITY,
            world,
        )?;
        data_buffers.create_uniform::<GpuPointLightMatrix>(
            "Point Light Matrix Buffer",
            INITIAL_CAPACITY,
            world,
        )
    }
}
