use crate::render::frame_graph::FrameGraphResourceId;

mod create_objects;
mod execute;
mod new;
mod write_resources;

/// A node that renders unlit objects using a forward rendering pipeline
#[derive(Debug)]
pub(in crate::render::frame_graph) struct UnlitForwardRenderNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,

    /// The depth buffer to use for depth testing
    depth_buffer: FrameGraphResourceId,
}
