//! The renderable definitions accessible to games

use colosseum_ecs::World;

mod anti_aliasing;
mod color_correction;
mod shader;
mod skybox;

pub use anti_aliasing::*;
pub use color_correction::*;
pub use shader::*;
pub use skybox::*;

/// Create the public facing resources
pub(in crate::render) fn create_public_data_resources(world: &mut World) {
    world.insert_resource(ColorCorrection::new());
    world.insert_resource(Skybox::default());
}
