//! The renderable definitions accessible to games

use crate::{
    FileIo,
    render::{GpuTransferQueue, GraphicsDevice},
};
use colosseum_ecs::World;
use std::sync::Arc;

mod anti_aliasing;
mod asset_loader;
mod color_correction;
mod shader;
mod skybox;

pub use anti_aliasing::*;
pub use asset_loader::*;
pub use color_correction::*;
pub use shader::*;
pub use skybox::*;

/// Create the public facing resources
pub(in crate::render) fn create_public_data_resources(
    world: &mut World,
    device: &Arc<GraphicsDevice>,
    gpu_transfer_queue: GpuTransferQueue,
) {
    let file_io = world.get_resource::<FileIo>().clone();

    world.insert_resource(AssetLoader::new(device, gpu_transfer_queue, file_io));
    world.insert_resource(ColorCorrection::new());
    world.insert_resource(Skybox::default());
}
