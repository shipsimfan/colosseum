use crate::render::frame_graph::{
    FrameGraphPipelineBarrier, FrameGraphResourceId, FrameGraphResourceState,
    FrameGraphTransientResourceInfo, builder::FrameGraphBuilderResources,
};
use alexandria::gpu::{VulkanAccessFlag, VulkanImageLayout, VulkanPipelineStageFlag};

impl FrameGraphBuilderResources {
    /// Finalize the resources used by the frame graph builder
    pub fn finish(
        mut self,
    ) -> (
        Option<FrameGraphPipelineBarrier>,
        Vec<FrameGraphTransientResourceInfo>,
    ) {
        let present_barrier = self.external_resources[0].transition(
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            FrameGraphResourceState::new(
                VulkanPipelineStageFlag::ColorAttachmentOutput,
                VulkanAccessFlag::ColorAttachmentWrite,
                VulkanImageLayout::PresentSrcKhr,
            ),
        );

        (present_barrier, self.native_scale_resources)
    }
}
