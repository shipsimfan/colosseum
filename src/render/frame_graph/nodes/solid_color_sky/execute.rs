use crate::render::{RenderData, RenderMaterial, frame_graph::SolidColorSkyNode};
use alexandria::{SlotMap, gpu::VulkanCommandBuffer, math::Vector2u};

impl SolidColorSkyNode {
    /// Execute the solid color sky pass, rendering a full-screen quad with the specified clear color
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        swapchain_size: Vector2u,
        cmd_buffer: &mut VulkanCommandBuffer,

        _: &SlotMap<RenderMaterial>,
    ) {
        // TODO: Draw a quad with the clear color to clear the render target
    }
}
