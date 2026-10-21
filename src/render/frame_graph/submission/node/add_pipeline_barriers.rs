use crate::render::{
    FrameIndex,
    frame_graph::{FrameGraphPipelineBarrier, FrameGraphResources, SubmissionNode},
};
use alexandria::gpu::VulkanCommandBuffer;

impl SubmissionNode {
    /// Add the pipeline barriers associated with this submission node to the primary command buffer
    pub fn add_pipeline_barriers(
        &self,
        primary_command_buffer: &mut VulkanCommandBuffer,
        barriers: &[FrameGraphPipelineBarrier],
        resources: &FrameGraphResources,
        frame_index: &FrameIndex,
    ) {
        for i in 0..self.pipeline_barriers {
            primary_command_buffer.cmd_pipeline_barrier2(
                0,
                &[],
                &[],
                &[barriers[i].barrier(resources, frame_index)],
            );
        }
    }
}
