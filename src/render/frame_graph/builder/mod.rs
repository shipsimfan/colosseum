use crate::render::{
    GraphicsDevice,
    frame_graph::{FrameGraphPipelineBarrier, FrameGraphResourceState, SubmissionNode},
};
use colosseum_ecs::Commands;

mod access;
mod add_node;
mod finish;
mod new;

/// Builder for constructing a frame graph
pub(in crate::render::frame_graph) struct FrameGraphBuilder<'a> {
    /// The command buffer being used for recording
    commands: &'a mut Commands,

    /// The graphics device to create objects with
    device: &'a GraphicsDevice,

    /// The nodes that will be used by the submission system
    nodes: Vec<SubmissionNode>,

    /// The current number of pipeline barriers needed for the current node
    current_pipeline_barriers: usize,

    /// The required pipeline barriers
    pipeline_barriers: Vec<FrameGraphPipelineBarrier>,

    /// The state of external resources
    external_resources: Vec<FrameGraphResourceState>,
}
