//! The system which submits the individual secondary command buffers to the primary command buffer
//! with appropriate pipeline barriers

use crate::render::{FrameIndex, RenderCommandBuffer, frame_graph::FrameGraphResources};

mod node;
mod pipeline_barrier;

pub(in crate::render::frame_graph) use node::*;
pub(in crate::render::frame_graph) use pipeline_barrier::*;

pub(in crate::render::frame_graph) fn submission_system(
    nodes: &[SubmissionNode],
    barriers: &[FrameGraphPipelineBarrier],
    resources: &FrameGraphResources,
    frame_index: &FrameIndex,
    primary_command_buffer: &mut RenderCommandBuffer,
) {
    let primary_command_buffer = &mut primary_command_buffer[frame_index];

    for node in nodes {
        // Add pipeline barriers
        node.add_pipeline_barriers(primary_command_buffer, barriers, &resources);

        // Submit the secondary command buffer to the primary command buffer
        node.submit(primary_command_buffer);
    }
}
