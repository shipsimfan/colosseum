use crate::render::frame_graph::FrameGraphResourceId;

mod create_objects;
mod execute;
mod new;
mod usages;

/// Change the render scale of the input image and output it to a new image
#[derive(Debug)]
pub(in crate::render) struct RenderScaleNode {
    /// The ID of the input image
    input: FrameGraphResourceId,

    /// The ID of the output image
    output: FrameGraphResourceId,
}
