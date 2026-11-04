use crate::{
    Result,
    render::{
        GpuDataBuffers, SpotLight,
        public_data::lights::spot::{GpuSpotLight, GpuSpotLightMatrix},
    },
};
use colosseum_ecs::World;

const INITIAL_CAPACITY: usize = 4;

impl SpotLight {
    /// Create the GPU data buffer for spot lights
    pub(in crate::render) fn create_buffer(
        data_buffers: &mut GpuDataBuffers,
        world: &mut World,
    ) -> Result<()> {
        data_buffers.create_uniform::<GpuSpotLight>(
            "Spot Light Buffer",
            INITIAL_CAPACITY,
            world,
        )?;
        data_buffers.create_uniform::<GpuSpotLightMatrix>(
            "Spot Light Matrix Buffer",
            INITIAL_CAPACITY,
            world,
        )
    }
}
