use crate::render::frame_graph::resources::resource::FrameGraphResourceState;
use alexandria::gpu::{VulkanAccessFlags, VulkanImageLayout, VulkanPipelineStageFlags};

impl FrameGraphResourceState {
    /// Get the current state of the resource as a tuple of (stage mask, access mask, layout)
    pub fn get(
        &self,
    ) -> (
        VulkanPipelineStageFlags,
        VulkanAccessFlags,
        VulkanImageLayout,
    ) {
        (self.stage_mask, self.access_mask, self.layout)
    }
}
