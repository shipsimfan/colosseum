use alexandria::gpu::{VulkanAccessFlags, VulkanImageLayout, VulkanPipelineStageFlags};

mod get;
mod new;
mod set;

/// The current state of a resource
pub(in crate::render::frame_graph::resources::resource) struct FrameGraphResourceState {
    /// The current stage mask for the resource
    stage_mask: VulkanPipelineStageFlags,

    /// The current access mask for the resource
    access_mask: VulkanAccessFlags,

    /// The current layout of the resource
    layout: VulkanImageLayout,
}
