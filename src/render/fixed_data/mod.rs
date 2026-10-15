use crate::{
    Result,
    render::{GraphicsDevice, create_frame_graph_fixed_resources},
};
use colosseum_ecs::World;

mod frame_index;
mod fullscreen_quad_shader;
mod pipeline;

pub(in crate::render) use frame_index::*;
pub(in crate::render) use fullscreen_quad_shader::*;
pub(in crate::render) use pipeline::*;

/// Create the fixed render data resources
pub(in crate::render) fn create_fixed_render_data_resources(
    world: &mut World,
    device: &GraphicsDevice,
) -> Result<()> {
    world.insert_resource(FrameIndex::new());
    world.insert_resource(FullscreenQuadShader::new(device)?);

    create_frame_graph_fixed_resources(world, device)
}
