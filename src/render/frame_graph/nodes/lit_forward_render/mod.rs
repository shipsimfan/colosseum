use crate::render::frame_graph::FrameGraphResourceId;

mod create_objects;
mod execute;
mod new;
mod usages;

/// A node that renders lit objects using a forward rendering pipeline
#[derive(Debug)]
pub(in crate::render::frame_graph) struct LitForwardRenderNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,

    /// The depth buffer to use for depth testing
    depth_buffer: FrameGraphResourceId,
}
