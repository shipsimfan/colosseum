use crate::render::frame_graph::FrameGraphResourceState;
use alexandria::gpu::{VulkanAccessFlags, VulkanImageLayout, VulkanPipelineStageFlags};

impl FrameGraphResourceState {
    /// Create a new [`FrameGraphResourceState`] with the specified values
    pub fn new<F1: Into<VulkanPipelineStageFlags>, F2: Into<VulkanAccessFlags>>(
        stage_mask: F1,
        access_mask: F2,
        layout: VulkanImageLayout,
    ) -> FrameGraphResourceState {
        FrameGraphResourceState {
            stage_mask: stage_mask.into(),
            access_mask: access_mask.into(),
            layout,
        }
    }
}
