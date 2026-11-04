use crate::{
    Result,
    render::{
        Camera, DirectionalLight, GpuDataBuffers, GraphicsDevice, LightMetadata, PointLight,
        Renderer, SpotLight,
    },
};
use colosseum_ecs::World;
use std::sync::Arc;

impl GpuDataBuffers {
    /// Create a new set of [`GpuDataBuffers`]
    pub(in crate::render::per_frame_data) fn new(
        device: Arc<GraphicsDevice>,
        world: &mut World,
    ) -> Result<()> {
        // Create the `GpuDataBuffers` struct
        let mut data_buffers = GpuDataBuffers {
            device,
            buffers: Vec::new(),
        };

        // Create the fixed data buffers
        Camera::create_buffer(&mut data_buffers, world)?;
        Renderer::create_buffer(&mut data_buffers, world)?;
        LightMetadata::create_buffer(&mut data_buffers, world)?;
        DirectionalLight::create_buffer(&mut data_buffers, world)?;
        PointLight::create_buffer(&mut data_buffers, world)?;
        SpotLight::create_buffer(&mut data_buffers, world)?;

        // Insert the created data buffers into the world
        world.insert_permanent_resource(data_buffers);
        Ok(())
    }
}
