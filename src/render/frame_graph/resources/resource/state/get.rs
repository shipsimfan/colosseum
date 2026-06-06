use crate::render::frame_graph::FrameGraphResourceState;
use alexandria::gpu::{VulkanAccessFlags, VulkanImageLayout, VulkanPipelineStageFlags};

impl FrameGraphResourceState {
    /// Get the current stage mask of the resource
    pub fn stage_mask(&self) -> VulkanPipelineStageFlags {
        self.stage_mask
    }

    /// Get the current access mask of the resource
    pub fn access_mask(&self) -> VulkanAccessFlags {
        self.access_mask
    }

    /// Get the current layout of the resource
    pub fn layout(&self) -> VulkanImageLayout {
        self.layout
    }
}
