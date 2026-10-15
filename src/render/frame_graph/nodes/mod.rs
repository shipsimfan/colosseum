use crate::{Result, render::GraphicsDevice};
use colosseum_ecs::World;

mod solid_color_sky;

pub(in crate::render::frame_graph) use solid_color_sky::*;

/// Creates the fixed resources for the frame graph
pub(in crate::render) fn create_frame_graph_fixed_resources(
    world: &mut World,
    device: &GraphicsDevice,
) -> Result<()> {
    SolidColorSkyNode::create_fixed_resources(world, device)
}
