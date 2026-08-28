use crate::render::frame_graph::FrameGraphResourceId;

mod create_objects;
mod execute;
mod new;
mod update_descriptor_sets;
mod usages;

/// Performs color correction, tone mapping, gamma correction, and color grading
#[derive(Debug)]
pub(in crate::render) struct ToneMapNode {
    /// The ID of the input image
    input: FrameGraphResourceId,

    /// The ID of the output image
    output: FrameGraphResourceId,
}

#[repr(C)]
struct PushConstants {
    contrast: f32,
    saturation: f32,
    exposure: f32,
    gamma: f32,
}
