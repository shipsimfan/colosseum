use crate::render::frame_graph::resources::resource::FrameGraphResourceState;
use alexandria::gpu::{VulkanAccessFlags, VulkanImageLayout, VulkanPipelineStageFlags};

impl FrameGraphResourceState {
    /// Transition the resource to a new state, returning the old state
    pub fn transition(
        &mut self,
        new_stage_mask: VulkanPipelineStageFlags,
        new_access_mask: VulkanAccessFlags,
        new_layout: VulkanImageLayout,
    ) -> (
        VulkanPipelineStageFlags,
        VulkanAccessFlags,
        VulkanImageLayout,
    ) {
        let old_stage_mask = self.stage_mask;
        let old_access_mask = self.access_mask;
        let old_layout = self.layout;

        self.stage_mask = new_stage_mask;
        self.access_mask = new_access_mask;
        self.layout = new_layout;

        (old_stage_mask, old_access_mask, old_layout)
    }
}
