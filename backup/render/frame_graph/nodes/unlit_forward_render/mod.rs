use crate::render::frame_graph::FrameGraphResourceId;

mod copy_data;
mod create_fixed_objects;
mod create_per_frame_objects;
mod execute;
mod new;
mod usages;

/// A node that renders unlit objects using a forward rendering pipeline
#[derive(Debug)]
pub(in crate::render::frame_graph) struct UnlitForwardRenderNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,

    /// The depth buffer to use for depth testing
    depth_buffer: FrameGraphResourceId,
}
