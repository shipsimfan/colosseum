use crate::render::frame_graph::FrameGraphResourceId;

mod create_fixed_objects;
mod create_per_frame_objects;
mod execute;
mod new;
mod usages;

/// Draws a solid color to the output, which can be used as the sky in a scene
#[derive(Debug)]
pub(in crate::render::frame_graph) struct SolidColorSkyNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,

    /// The depth buffer to use for depth testing
    depth_buffer: FrameGraphResourceId,
}
