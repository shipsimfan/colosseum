use crate::render::{RenderData, frame_graph::SolidColorSkyNode};
use alexandria::gpu::VulkanCommandBuffer;

impl SolidColorSkyNode {
    /// Execute the unlit forward pass, rendering all unlit objects in the scene
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        // TODO: Draw a quad with the clear color to clear the render target
    }
}
