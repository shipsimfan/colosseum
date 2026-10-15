//! The renderable definitions accessible to games

use colosseum_ecs::World;

mod shader;
mod skybox;

pub use shader::*;
pub use skybox::*;

/// Create the public facing resources
pub(in crate::render) fn create_render_data_resources(world: &mut World) {
    world.insert_resource(Skybox::default());
}
