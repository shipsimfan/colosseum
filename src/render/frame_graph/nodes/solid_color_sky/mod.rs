use crate::render::frame_graph::FrameGraphResourceId;

mod add;
mod create_resources;
mod node;
mod system;

/// Draws a solid color to the output, which can be used as the sky in a scene
#[derive(Debug)]
pub(in crate::render::frame_graph) struct SolidColorSkyNode {
    /// The output resource ID for the solid color sky node
    output: FrameGraphResourceId,
}
