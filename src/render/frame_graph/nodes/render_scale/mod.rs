use crate::render::frame_graph::FrameGraphResourceId;

mod add;
mod node;
mod system;

/// Change the render scale of the input image and output it to a new image
#[derive(Debug)]
pub(in crate::render::frame_graph) struct RenderScaleNode {
    /// The input image resource ID
    input: FrameGraphResourceId,

    /// The output image resource ID
    output: FrameGraphResourceId,
}
