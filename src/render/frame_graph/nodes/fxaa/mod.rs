use crate::render::frame_graph::FrameGraphResourceId;

mod add;
mod create_resources;
mod node;
mod system;

/// Performs FXAA anti-aliasing on an input image
#[derive(Debug)]
pub(in crate::render::frame_graph) struct FxaaNode {
    /// The index of the descriptor set used for the input image
    descriptor_set: usize,

    /// The output image
    output: FrameGraphResourceId,
}
