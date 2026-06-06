use crate::render::{RenderData, frame_graph::UnlitForwardRenderNode};
use alexandria::gpu::VulkanCommandBuffer;

impl UnlitForwardRenderNode {
    /// Execute the unlit forward pass, rendering all unlit objects in the scene
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
    }
}
