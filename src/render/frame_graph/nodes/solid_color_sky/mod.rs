use crate::render::frame_graph::FrameGraphResourceId;

mod create_objects;
mod execute;
mod new;
mod write_resources;

/// Draws a solid color to the output, which can be used as the sky in a scene
#[derive(Debug)]
pub(in crate::render) struct SolidColorSkyNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,
}
