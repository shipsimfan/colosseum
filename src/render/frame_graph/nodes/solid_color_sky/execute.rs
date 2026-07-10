use crate::render::{RenderData, RenderObjects, frame_graph::SolidColorSkyNode};
use alexandria::{gpu::VulkanCommandBuffer, math::Vector2u};

impl SolidColorSkyNode {
    /// Execute the solid color sky pass, rendering a full-screen quad with the specified clear color
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        swapchain_size: Vector2u,
        cmd_buffer: &mut VulkanCommandBuffer,
        render_objects: &RenderObjects,
    ) {
        // TODO: Draw a quad with the clear color to clear the render target
    }
}
