use crate::{
    Result,
    render::{GraphicsDevice, LitForwardMaterial, UnlitMaterial},
};
use colosseum_ecs::World;

mod descriptor_set_layouts;
mod frame_index;
mod fullscreen_quad_shader;
mod render_queue;
mod renderables;
mod samplers;

pub(in crate::render) use descriptor_set_layouts::*;
pub(in crate::render) use frame_index::*;
pub(in crate::render) use fullscreen_quad_shader::*;
pub(in crate::render) use render_queue::*;
pub(in crate::render) use renderables::*;
pub(in crate::render) use samplers::*;

/// Create the fixed render data resources
pub(in crate::render) fn create_fixed_render_data_resources(
    world: &mut World,
    device: &GraphicsDevice,
) -> Result<()> {
    world.insert_permanent_resource(FrameIndex::new());
    world.insert_permanent_resource(FullscreenQuadShader::new(device)?);
    world.insert_permanent_resource(Samplers::new(device)?);
    world.insert_permanent_resource(DescriptorSetLayouts::new(device)?);

    world.insert_permanent_resource(Renderables::<UnlitMaterial>::new());
    world.insert_permanent_resource(Renderables::<LitForwardMaterial>::new());

    Ok(())
}
