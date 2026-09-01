use crate::render::frame_graph::FrameGraphResourceId;

mod create_fixed_objects;
mod create_per_frame_objects;
mod execute;
mod new;
mod update_descriptor_sets;
mod usages;

/// Performs FXAA anti-aliasing on an input image
#[derive(Debug)]
pub(in crate::render::frame_graph) struct FxaaNode {
    /// The ID of the input image
    input: FrameGraphResourceId,

    /// The ID of the output image
    output: FrameGraphResourceId,
}
