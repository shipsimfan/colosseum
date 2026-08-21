use crate::render::frame_graph::{FrameGraphPipelineBarrier, FrameGraphResources};
use alexandria::gpu::{VK_QUEUE_FAMILY_IGNORED, VulkanImageMemoryBarrier};

impl FrameGraphPipelineBarrier {
    /// Create a [`VulkanImageMemoryBarrier`] for this pipeline barrier
    pub fn barrier<'a>(&self, resources: &'a FrameGraphResources) -> VulkanImageMemoryBarrier<'a> {
        let resource = resources.get(self.resource);

        VulkanImageMemoryBarrier::new(
            self.old_state.stage_mask(),
            self.old_state.access_mask(),
            self.new_state.stage_mask(),
            self.new_state.access_mask(),
            self.old_state.layout(),
            self.new_state.layout(),
            VK_QUEUE_FAMILY_IGNORED,
            VK_QUEUE_FAMILY_IGNORED,
            resource.image(),
            resource.aspect_mask(),
            0,
            1,
            0,
            1,
        )
    }
}
