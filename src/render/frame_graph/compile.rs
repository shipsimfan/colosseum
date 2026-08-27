use crate::render::{
    FrameGraph,
    frame_graph::{
        FrameGraphNode, FrameGraphPipelineBarrier, FrameGraphResourceBuilder,
        FrameGraphResourceState, FrameGraphResourceUsage,
    },
};
use alexandria::gpu::{VulkanAccessFlag, VulkanImageLayout, VulkanPipelineStageFlag};

impl FrameGraph {
    /// Compile the frame graph, preparing it for execution
    pub(in crate::render::frame_graph) fn compile<'a>(
        nodes: &[FrameGraphNode],
        resource_builder: &mut FrameGraphResourceBuilder<'a>,

        pipeline_barrier_indices: &mut Vec<(usize, usize)>,
        pipeline_barriers: &mut Vec<FrameGraphPipelineBarrier>,
    ) {
        // Reset the pipeline barriers for this frame
        pipeline_barrier_indices.clear();
        pipeline_barriers.clear();

        // Create a pipeline barrier for each transition needed for each node in the frame graph
        for (node_index, node) in nodes.iter().enumerate() {
            node.usages(|write_resources| {
                let mut num_barriers = 0;
                for (id, usage) in write_resources {
                    let new_state = match usage {
                        FrameGraphResourceUsage::ColorAttachment => {
                            resource_builder.set_color(*id);
                            FrameGraphResourceState::new(
                                VulkanPipelineStageFlag::ColorAttachmentOutput,
                                VulkanAccessFlag::ColorAttachmentWrite,
                                VulkanImageLayout::ColorAttachmentOptimal,
                            )
                        }
                        FrameGraphResourceUsage::DepthAttachment => {
                            resource_builder.set_depth(*id);
                            FrameGraphResourceState::new(
                                VulkanPipelineStageFlag::EarlyFragmentTests,
                                VulkanAccessFlag::DepthStencilAttachmentWrite,
                                VulkanImageLayout::DepthStencilAttachmentOptimal,
                            )
                        }
                        FrameGraphResourceUsage::TransferDst => {
                            resource_builder.set_transfer_dst(*id);
                            FrameGraphResourceState::new(
                                VulkanPipelineStageFlag::Blit,
                                VulkanAccessFlag::TransferWrite,
                                VulkanImageLayout::TransferDstOptimal,
                            )
                        }
                        FrameGraphResourceUsage::TransferSrc => {
                            resource_builder.set_transfer_src(*id);
                            FrameGraphResourceState::new(
                                VulkanPipelineStageFlag::Blit,
                                VulkanAccessFlag::TransferRead,
                                VulkanImageLayout::TransferSrcOptimal,
                            )
                        }
                        FrameGraphResourceUsage::SampledImage => {
                            resource_builder.set_sampled_image(*id);
                            FrameGraphResourceState::new(
                                VulkanPipelineStageFlag::FragmentShader,
                                VulkanAccessFlag::ShaderRead,
                                VulkanImageLayout::ShaderReadOnlyOptimal,
                            )
                        }
                    };

                    if let Some(barrier) = resource_builder.transition(*id, new_state) {
                        pipeline_barriers.push(barrier);
                        num_barriers += 1;
                    }
                }

                if num_barriers > 0 {
                    pipeline_barrier_indices.push((node_index, num_barriers));
                }
            });
        }
    }
}
