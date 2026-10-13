use crate::render::frame_graph::FrameGraphResourceId;

mod create_fixed_objects;
mod create_per_frame_objects;
mod execute;
mod new;
mod update_descriptor_sets;
mod usages;

/// Change the render scale of the input image and output it to a new image
#[derive(Debug)]
pub(in crate::render::frame_graph) struct RenderScaleNode {
    /// The ID of the input image
    input: FrameGraphResourceId,

    /// The ID of the output image
    output: FrameGraphResourceId,
}
