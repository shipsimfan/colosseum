use crate::{
    Result,
    render::{FrameGraph, GraphicsDevice, frame_graph::FrameGraphResources},
};
use colosseum_ecs::World;
use std::sync::Arc;

mod node;
mod system;

mod fxaa;
mod quantization;
mod render_scale;
mod solid_color_sky;
mod tone_map;

pub(in crate::render::frame_graph) use node::*;
pub(in crate::render::frame_graph) use system::*;

pub(in crate::render::frame_graph) use fxaa::*;
pub(in crate::render::frame_graph) use quantization::*;
pub(in crate::render::frame_graph) use render_scale::*;
pub(in crate::render::frame_graph) use solid_color_sky::*;
pub(in crate::render::frame_graph) use tone_map::*;

/// Creates the fixed resources for the frame graph
pub(in crate::render) fn create_frame_graph_resources<Game: crate::Game>(
    world: &mut World,
    device: &Arc<GraphicsDevice>,
) -> Result<()> {
    world.insert_permanent_resource(FrameGraphResources::empty());
    world.insert_permanent_resource(FrameGraph::<Game>::new(device));

    FxaaNode::create_resources(world, device)?;
    QuantizationNode::create_resources(world, device)?;
    SolidColorSkyNode::create_resources(world, device)?;
    ToneMapNode::<Game>::create_resources(world, device)
}
