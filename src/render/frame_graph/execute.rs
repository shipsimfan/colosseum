use crate::render::{
    FrameGraph, RenderData, RenderMaterial,
    frame_graph::{
        ArenaBuffer, FrameGraphNode, FrameGraphPipelineBarrier, FrameGraphResourceId,
        FrameGraphResourceState, FrameGraphResourceWriteUsage, FrameGraphResources,
    },
};
use alexandria::{
    SlotMap,
    gpu::{
        VulkanAccessFlag, VulkanAttachmentStoreOp, VulkanCommandBuffer, VulkanImageLayout,
        VulkanImageMemoryBarrier, VulkanPipelineStageFlag, VulkanRenderingAttachmentInfo,
        VulkanResolveModeFlag,
    },
    math::{Color4u, Linear, Vector2i, Vector2u},
};

impl FrameGraph {
    /// Execute the frame graph, rendering a frame
    pub(in crate::render::frame_graph) fn execute(
        data: &RenderData,
        resources: &FrameGraphResources,
        nodes: &[FrameGraphNode],

        pipeline_barrier_indices: &[(usize, usize)],
        pipeline_barriers: &[FrameGraphPipelineBarrier],

        image_barriers: &mut ArenaBuffer<VulkanImageMemoryBarrier<'static>>,
        color_attachments: &mut ArenaBuffer<VulkanRenderingAttachmentInfo<'static>>,
        cmd_buffer: &mut VulkanCommandBuffer,

        swapchain_size: Vector2u,
        materials: &SlotMap<RenderMaterial>,
    ) {
        let mut current_pipeline_barrier_index = 0;
        let mut current_pipeline_barrier = 0;

        // Execute each node
        //
        // TODO: execute nodes in topological order
        for (index, node) in nodes.iter().enumerate() {
            // Collect any pipeline barriers needed for this node and execute them
            if current_pipeline_barrier_index < pipeline_barrier_indices.len()
                && pipeline_barrier_indices[current_pipeline_barrier_index].0 == index
            {
                let mut image_barriers = image_barriers.arena();
                for _ in 0..pipeline_barrier_indices[current_pipeline_barrier_index].1 {
                    let barrier = &pipeline_barriers
                        [current_pipeline_barrier + current_pipeline_barrier_index];
                    image_barriers.push(barrier.barrier(resources));
                }

                cmd_buffer.cmd_pipeline_barrier2(0, &[], &[], image_barriers.as_slice());

                current_pipeline_barrier +=
                    pipeline_barrier_indices[current_pipeline_barrier_index].1;
                current_pipeline_barrier_index += 1;
            }

            // Create attachment infos for all output resources of the node
            let mut color_attachments = color_attachments.arena();
            let resource_size = node.write_resources(|write_resources| {
                for (id, usage) in write_resources {
                    let resource = resources.get(*id);

                    match usage {
                        FrameGraphResourceWriteUsage::ColorAttachment { load_op } => {
                            color_attachments.push(VulkanRenderingAttachmentInfo::new(
                                resource.image_view(),
                                VulkanImageLayout::ColorAttachmentOptimal,
                                VulkanResolveModeFlag::None,
                                None,
                                VulkanImageLayout::Undefined,
                                *load_op,
                                VulkanAttachmentStoreOp::Store,
                                Color4u::<Linear>::new(0, 0, 0, 0),
                            ));
                        }
                    }
                }

                resources.get(write_resources[0].0).size()
            });

            // Begin rendering
            if color_attachments.len() > 0 {
                cmd_buffer.cmd_begin_rendering(
                    0,
                    Vector2i::ZERO,
                    resource_size,
                    1,
                    0,
                    color_attachments.as_slice(),
                    None,
                    None,
                );
            }

            // Execute the node
            node.execute(data, swapchain_size, cmd_buffer, materials);

            // End rendering
            cmd_buffer.cmd_end_rendering();
        }

        // Queue a final barrier to transition the swapchain image to the present layout
        if let Some(present_barrier) = FrameGraphPipelineBarrier::new(
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            resources
                .get(FrameGraphResourceId::SWAPCHAIN_IMAGE)
                .state()
                .clone(),
            FrameGraphResourceState::new(
                VulkanPipelineStageFlag::ColorAttachmentOutput,
                VulkanAccessFlag::ColorAttachmentWrite,
                VulkanImageLayout::PresentSrcKhr,
            ),
        ) {
            let image_barrier = present_barrier.barrier(resources);
            cmd_buffer.cmd_pipeline_barrier2(0, &[], &[], &[image_barrier]);
        }
    }
}
