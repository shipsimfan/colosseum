use crate::render::{
    FrameGraph, RenderData,
    frame_graph::{FrameGraphResourceWriteUsage, FrameGraphResources},
};
use alexandria::{
    gpu::{
        VulkanAccessFlag, VulkanAttachmentStoreOp, VulkanCommandBuffer, VulkanImageLayout,
        VulkanImageMemoryBarrier, VulkanPipelineStageFlag, VulkanRenderingAttachmentInfo,
        VulkanResolveModeFlag,
    },
    math::Vector2i,
};

impl FrameGraph {
    /// Execute the frame graph, rendering a frame
    pub(in crate::render::frame_graph) fn execute(
        &mut self,
        data: &RenderData,
        cmd_buffer: &mut VulkanCommandBuffer,
        resources: &FrameGraphResources,
    ) {
        // Convert `'static` buffers to `'_` buffers for use in the command buffer
        let image_barriers: &mut Vec<VulkanImageMemoryBarrier<'_>> =
            unsafe { std::mem::transmute(&mut self.image_barriers_buffer) };
        let color_attachments: &mut Vec<VulkanRenderingAttachmentInfo<'_>> =
            unsafe { std::mem::transmute(&mut self.color_attachments_buffer) };

        // TODO: execute nodes in topological order
        for node in &self.nodes {
            // TODO: Implement a proper resource state tracking system to minimize unnecessary pipeline barriers and make correct resource transitions

            // Create barriers and attachment infos for all output resources of the node, and ensure that they all have the same size
            let resource_size = node.write_resources(|write_resources| {
                let resource_size = resources[write_resources[0].0].size();
                for (id, usage) in write_resources {
                    let resource = &resources[*id];
                    debug_assert!(
                        resource.size() == resource_size,
                        "all output resources of a node must have the same size"
                    );

                    match usage {
                        FrameGraphResourceWriteUsage::ColorAttachment { load_op } => {
                            image_barriers.push(resource.barrier(
                                VulkanImageLayout::ColorAttachmentOptimal,
                                VulkanPipelineStageFlag::ColorAttachmentOutput,
                                VulkanAccessFlag::ColorAttachmentWrite,
                            ));

                            let (load_op, clear_value) = load_op.to_vk();
                            color_attachments.push(VulkanRenderingAttachmentInfo::new(
                                resource.image_view(),
                                VulkanImageLayout::ColorAttachmentOptimal,
                                VulkanResolveModeFlag::None,
                                None,
                                VulkanImageLayout::Undefined,
                                load_op,
                                VulkanAttachmentStoreOp::Store,
                                clear_value,
                            ));
                        }
                    }
                }

                resource_size
            });

            // Place the pipeline barriers
            if image_barriers.len() > 0 {
                cmd_buffer.cmd_pipeline_barrier2(0, &[], &[], image_barriers);
            }
            image_barriers.clear();

            // Begin rendering
            if color_attachments.len() > 0 {
                cmd_buffer.cmd_begin_rendering(
                    0,
                    Vector2i::ZERO,
                    resource_size,
                    1,
                    0,
                    color_attachments,
                    None,
                    None,
                );
            }
            color_attachments.clear();

            node.execute(data, cmd_buffer);

            cmd_buffer.cmd_end_rendering();
        }
    }
}
