use crate::render::{RenderData, frame_graph::SolidColorSkyNode};
use alexandria::gpu::VulkanCommandBuffer;

impl SolidColorSkyNode {
    /// Execute the solid color sky pass, rendering a full-screen quad with the specified clear color
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        // TODO: Draw a quad with the clear color to clear the render target
    }
}
