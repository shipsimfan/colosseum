use crate::render::{
    frame_graph::{
        FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceState, FrameGraphResources,
        FrameGraphSubmissionSystem,
    },
    phases::FrameGraph,
};
use alexandria::gpu::{VulkanAccessFlag, VulkanImageLayout, VulkanPipelineStageFlag};

impl<'a> FrameGraphBuilder<'a> {
    /// Register the submission system
    pub fn finish(mut self, resources: &mut FrameGraphResources) {
        // Add a final pipeline barrier to prepare the swapchain image for present
        if let Some(barrier) = self.external_resources[0].transition(
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            FrameGraphResourceState::new(
                VulkanPipelineStageFlag::ColorAttachmentOutput,
                VulkanAccessFlag::ColorAttachmentWrite,
                VulkanImageLayout::PresentSrcKhr,
            ),
        ) {
            self.pipeline_barriers.push(barrier);
        }

        // Insert the new resource instance into the world
        resources.clear();
        *resources = FrameGraphResources::new(self.native_scale_resources);

        // Register submission system
        self.commands.register_system_direct(
            FrameGraph,
            FrameGraphSubmissionSystem::new(self.nodes, self.pipeline_barriers),
        );
    }
}
