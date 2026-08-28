use crate::render::frame_graph::FrameGraphResourceId;
use alexandria::math::Vector2f;

mod create_objects;
mod execute;
mod new;
mod update_descriptor_sets;
mod usages;

/// Sharpens and applies dithering to an image before quantizing it
#[derive(Debug)]
pub(in crate::render) struct QuantizationNode {
    /// The ID of the input image
    input: FrameGraphResourceId,

    /// The ID of the output image
    output: FrameGraphResourceId,
}

#[repr(C)]
struct PushConstants {
    image_size: Vector2f,
    texel_size: Vector2f,
    sharpness: f32,
}
