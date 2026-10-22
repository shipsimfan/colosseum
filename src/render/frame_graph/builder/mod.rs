use crate::render::{
    DescriptorSetBinding, GraphicsDevice,
    frame_graph::{FrameGraphPipelineBarrier, FrameGraphResourceId, SubmissionNode},
};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanDescriptorPoolSize};
use colosseum_ecs::Commands;
use resource::*;
use resources::*;
use std::{any::TypeId, borrow::Cow};

mod resource;
mod resources;

mod access;
mod add_node;
mod create_descriptor_set;
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

    /// The count of descriptors used by the frame graph
    descriptors: Vec<VulkanDescriptorPoolSize>,

    /// The layouts of the descriptor sets used by the frame graph
    descriptor_sets: Vec<(TypeId, Cow<'static, str>, &'static [DescriptorSetBinding])>,

    /** Current Node Trackers **/

    /// The current number of pipeline barriers needed for the current node
    current_pipeline_barriers: usize,

    /// The color attachment for the current node
    color_attachment: Option<(FrameGraphResourceId, VulkanAttachmentLoadOp)>,
}
