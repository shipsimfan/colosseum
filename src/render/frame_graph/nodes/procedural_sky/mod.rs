use crate::render::frame_graph::FrameGraphResourceId;

mod create_fixed_objects;
mod create_per_frame_objects;
mod execute;
mod new;
mod usages;

/// Renders a procedural sky to the output
#[derive(Debug)]
pub(in crate::render::frame_graph) struct ProceduralSkyNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,
}
