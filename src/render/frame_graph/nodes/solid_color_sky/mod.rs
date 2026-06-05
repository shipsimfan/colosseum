use crate::render::frame_graph::FrameGraphResourceId;
use alexandria::math::{Color3f, Linear};

mod execute;
mod new;
mod write_resources;

/// Draws a solid color to the output, which can be used as the sky in a scene
#[derive(Debug)]
pub(in crate::render) struct SolidColorSkyNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,

    /// The color to clear the color attachment to
    ///
    /// TODO: Remove this and replace it with a quad that fills the screen when we support
    /// depth/stencil attachments, since we won't be able to clear those with a specific color
    color: Color3f<Linear>,
}
