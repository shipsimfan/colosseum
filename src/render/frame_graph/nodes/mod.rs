use crate::{
    Result,
    render::{FrameGraph, GraphicsDevice, frame_graph::FrameGraphResources},
};
use colosseum_ecs::World;

mod node;
mod solid_color_sky;
mod system;

pub(in crate::render::frame_graph) use node::*;
pub(in crate::render::frame_graph) use solid_color_sky::*;
pub(in crate::render::frame_graph) use system::*;

/// Creates the fixed resources for the frame graph
pub(in crate::render) fn create_frame_graph_resources(
    world: &mut World,
    device: &GraphicsDevice,
) -> Result<()> {
    world.insert_resource(FrameGraphResources::new());
    world.insert_resource(FrameGraph::new());

    SolidColorSkyNode::create_resources(world, device)
}
