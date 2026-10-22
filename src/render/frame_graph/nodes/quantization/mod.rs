use crate::render::frame_graph::FrameGraphResourceId;
use alexandria::math::Vector2f;

mod add;
mod create_resources;
mod node;
mod system;

/// Sharpens and applies dithering to an image before quantizing it
#[derive(Debug)]
pub(in crate::render::frame_graph) struct QuantizationNode {
    /// The index of the descriptor set used for the input image
    descriptor_set: usize,

    /// The output image
    output: FrameGraphResourceId,
}

#[repr(C)]
struct PushConstants {
    image_size: Vector2f,
    texel_size: Vector2f,
    sharpness: f32,
}
