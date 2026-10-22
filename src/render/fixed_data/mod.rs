use crate::{Result, render::GraphicsDevice};
use colosseum_ecs::World;

mod descriptor_set_layouts;
mod frame_index;
mod fullscreen_quad_shader;
mod post_process_descriptor_layout;
mod samplers;

pub(in crate::render) use descriptor_set_layouts::*;
pub(in crate::render) use frame_index::*;
pub(in crate::render) use fullscreen_quad_shader::*;
pub(in crate::render) use post_process_descriptor_layout::*;
pub(in crate::render) use samplers::*;

/// Create the fixed render data resources
pub(in crate::render) fn create_fixed_render_data_resources(
    world: &mut World,
    device: &GraphicsDevice,
) -> Result<()> {
    world.insert_resource(FrameIndex::new());
    world.insert_resource(FullscreenQuadShader::new(device)?);
    world.insert_resource(Samplers::new(device)?);

    let mut descriptor_set_layouts = DescriptorSetLayouts::new();
    PostProcess::new(&mut descriptor_set_layouts, device)?;
    world.insert_resource(descriptor_set_layouts);

    Ok(())
}
