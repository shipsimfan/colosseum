use crate::render::{FrameGraph, RenderData, frame_graph::FrameGraphResources};
use alexandria::gpu::{
    VulkanAccessFlag, VulkanAccessFlags, VulkanCommandBuffer, VulkanImageLayout,
    VulkanPipelineStageFlag,
};

impl FrameGraph {
    /// Execute the frame graph, rendering a frame
    pub(in crate::render::frame_graph) fn execute(
        &mut self,
        data: &RenderData,
        cmd_buffer: &mut VulkanCommandBuffer,
        resources: &FrameGraphResources,
    ) {
        // TODO: execute nodes in topological order
        for node in &self.nodes {
            // Transition the output resources of the node to the appropriate layout for writing
            //
            // TODO: Implement a proper resource state tracking system to minimize unnecessary pipeline barriers and make correct resource transitions
            node.write_resources(|write_resource_id| {
                let resource = &resources[write_resource_id];
                cmd_buffer.cmd_pipeline_barrier2(
                    resource.image(),
                    VulkanImageLayout::Undefined,
                    VulkanImageLayout::ColorAttachmentOptimal,
                    VulkanAccessFlags::default(),
                    VulkanAccessFlag::ColorAttachmentWrite,
                    VulkanPipelineStageFlag::ColorAttachmentOutput,
                    VulkanPipelineStageFlag::ColorAttachmentOutput,
                );
            });

            node.execute(data, resources, cmd_buffer);
        }
    }
}
