use crate::render::{
    FrameGraph,
    frame_graph::{
        FrameGraphNode, FrameGraphPipelineBarrier, FrameGraphResourceState,
        FrameGraphResourceWriteUsage, FrameGraphResources,
    },
};
use alexandria::gpu::{VulkanAccessFlag, VulkanImageLayout, VulkanPipelineStageFlag};

impl FrameGraph {
    /// Compile the frame graph, preparing it for execution
    pub(in crate::render::frame_graph) fn compile(
        nodes: &[FrameGraphNode],
        resources: &mut FrameGraphResources,

        pipeline_barrier_indices: &mut Vec<(usize, usize)>,
        pipeline_barriers: &mut Vec<FrameGraphPipelineBarrier>,
    ) {
        // Reset the pipeline barriers for this frame
        pipeline_barrier_indices.clear();
        pipeline_barriers.clear();

        // Create a pipeline barrier for each transition needed for each node in the frame graph
        for (node_index, node) in nodes.iter().enumerate() {
            node.write_resources(|write_resources| {
                let resource_size = resources.get(write_resources[0].0).size();
                let mut num_barriers = 0;
                for (id, usage) in write_resources {
                    let resource = resources.get(*id);
                    debug_assert!(
                        resource.size() == resource_size,
                        "all output resources of a node must have the same size"
                    );

                    let new_state = match usage {
                        FrameGraphResourceWriteUsage::ColorAttachment { load_op: _ } => {
                            FrameGraphResourceState::new(
                                VulkanPipelineStageFlag::ColorAttachmentOutput,
                                VulkanAccessFlag::ColorAttachmentWrite,
                                VulkanImageLayout::ColorAttachmentOptimal,
                            )
                        }
                    };

                    if let Some(barrier) = resources.transition(*id, new_state) {
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
