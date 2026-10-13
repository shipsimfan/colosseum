use alexandria::gpu::{VulkanAccessFlags, VulkanImageLayout, VulkanPipelineStageFlags};

mod default;
mod get;
mod new;
mod transition;

/// The current state of a resource
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::render::frame_graph) struct FrameGraphResourceState {
    /// The current stage mask for the resource
    stage_mask: VulkanPipelineStageFlags,

    /// The current access mask for the resource
    access_mask: VulkanAccessFlags,

    /// The current layout of the resource
    layout: VulkanImageLayout,
}
