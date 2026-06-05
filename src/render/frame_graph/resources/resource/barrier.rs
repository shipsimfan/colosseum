use crate::render::frame_graph::FrameGraphResource;
use alexandria::gpu::{
    VK_QUEUE_FAMILY_IGNORED, VulkanAccessFlags, VulkanImageAspectFlag, VulkanImageLayout,
    VulkanImageMemoryBarrier, VulkanPipelineStageFlags,
};

impl<'a> FrameGraphResource<'a> {
    /// Create a [`VulkanImageMemoryBarrier`] to transition this resource to the requested layout, stage mask, and access mask
    pub fn barrier<'b, F1: Into<VulkanPipelineStageFlags>, F2: Into<VulkanAccessFlags>>(
        &'b mut self,
        new_layout: VulkanImageLayout,
        new_stage_mask: F1,
        new_access_mask: F2,
    ) -> VulkanImageMemoryBarrier<'b> {
        VulkanImageMemoryBarrier::new(
            self.stage_mask,
            self.access_mask,
            new_stage_mask,
            new_access_mask,
            self.layout,
            new_layout,
            VK_QUEUE_FAMILY_IGNORED,
            VK_QUEUE_FAMILY_IGNORED,
            self.image_view.image(),
            VulkanImageAspectFlag::Color,
            0,
            1,
            0,
            1,
        )
    }
}
