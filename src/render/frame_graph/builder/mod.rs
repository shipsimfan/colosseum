use crate::render::{
    GraphicsDevice,
    frame_graph::{FrameGraphPipelineBarrier, FrameGraphResourceId, SubmissionNode},
};
use alexandria::gpu::VulkanAttachmentLoadOp;
use colosseum_ecs::Commands;
use resource::*;
use resources::*;

mod resource;
mod resources;

mod access;
mod add_node;
mod create_resource;
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

    /// The required pipeline barriers
    pipeline_barriers: Vec<FrameGraphPipelineBarrier>,

    /// The resources used by the frame graph
    resources: FrameGraphBuilderResources,

    /** Current Node Trackers **/

    /// The current number of pipeline barriers needed for the current node
    current_pipeline_barriers: usize,

    /// The color attachment for the current node
    color_attachment: Option<(FrameGraphResourceId, VulkanAttachmentLoadOp)>,
}
