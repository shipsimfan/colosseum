use crate::render::frame_graph::FrameGraphResourceId;

mod create_objects;
mod execute;
mod new;
mod update_descriptor_sets;
mod usages;

/// Sharpen, gamma correct, and apply dithering to the input image and output it to a new image
#[derive(Debug)]
pub(in crate::render) struct GammaCorrectionNode {
    /// The ID of the input image
    input: FrameGraphResourceId,

    /// The ID of the output image
    output: FrameGraphResourceId,
}
