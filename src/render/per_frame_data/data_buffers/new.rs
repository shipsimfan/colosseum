use crate::{
    Result,
    render::{Camera, GpuDataBuffers, GraphicsDevice},
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

        // Insert the created data buffers into the world
        world.insert_permanent_resource(data_buffers);
        Ok(())
    }
}
