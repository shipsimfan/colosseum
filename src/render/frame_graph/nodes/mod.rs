use crate::{
    Result,
    render::{FrameGraph, GraphicsDevice, frame_graph::FrameGraphResources},
};
use colosseum_ecs::World;

mod node;
mod system;

mod quantization;
mod solid_color_sky;

pub(in crate::render::frame_graph) use node::*;
pub(in crate::render::frame_graph) use system::*;

pub(in crate::render::frame_graph) use quantization::*;
pub(in crate::render::frame_graph) use solid_color_sky::*;

/// Creates the fixed resources for the frame graph
pub(in crate::render) fn create_frame_graph_resources(
    world: &mut World,
    device: &GraphicsDevice,
) -> Result<()> {
    world.insert_resource(FrameGraphResources::empty());
    world.insert_resource(FrameGraph::new());

    SolidColorSkyNode::create_resources(world, device)?;
    QuantizationNode::create_resources(world, device)
}
