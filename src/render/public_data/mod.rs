//! The renderable definitions accessible to games

use crate::{
    FileIo, Result,
    render::{DescriptorSetLayouts, GpuTransferQueue, GraphicsDevice, RenderGpuTransferQueue},
};
use alexandria::gpu::VulkanQueue;
use colosseum_ecs::World;
use std::sync::Arc;

mod anti_aliasing;
mod assets;
mod camera;
mod color_correction;
mod lights;
mod materials;
mod mesh;
mod primitives;
mod renderer;
mod skybox;

pub use anti_aliasing::*;
pub use assets::*;
pub use camera::*;
pub use color_correction::*;
pub use lights::*;
pub use materials::*;
pub use mesh::*;
pub use primitives::*;
pub use renderer::*;
pub use skybox::*;

/// Create the public facing resources
pub(in crate::render) fn create_public_data_resources(
    world: &mut World,
    device: &Arc<GraphicsDevice>,
    gpu_transfer_queue: GpuTransferQueue,
    render_gpu_transfer_queue: Option<(&mut RenderGpuTransferQueue, &mut VulkanQueue)>,
) -> Result<()> {
    let file_io = world.get_resource::<FileIo>().clone();

    let mut assets = Assets::new(device, gpu_transfer_queue, file_io);
    world.insert_permanent_resource(Primitives::new(&mut assets, render_gpu_transfer_queue)?);
    world.insert_permanent_resource(assets);

    let materials = Materials::new(&world.get_resource::<DescriptorSetLayouts>(), device)?;
    world.insert_permanent_resource(materials);

    Ok(())
}

/// Create the public facing scene-specific resources
pub(in crate::render) fn create_public_data_scene_resources(world: &mut World) {
    world.insert_resource(ColorCorrection::new());
    world.insert_resource(Skybox::default());
    world.insert_resource(Camera::default());
    world.insert_resource(AmbientLight::default());
}
