mod create_fixed_resources;
mod system;

/// Draws a solid color to the output, which can be used as the sky in a scene
#[derive(Debug)]
pub(in crate::render::frame_graph) struct SolidColorSkyNode {}
