use crate::render::{RenderData, RenderObjects, frame_graph::UnlitForwardRenderNode};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanViewport},
    math::{Recti, Vector2, Vector2u},
};

impl UnlitForwardRenderNode {
    /// Execute the unlit forward pass, rendering all unlit objects in the scene
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        swapchain_size: Vector2u,
        cmd_buffer: &mut VulkanCommandBuffer,
        render_objects: &RenderObjects,
    ) {
        // Bind the viewport and scissor for the render pass
        let viewport = VulkanViewport::new(
            0.0,
            0.0,
            swapchain_size.x as _,
            swapchain_size.y as _,
            0.0,
            1.0,
        );
        let scissor = Recti::new(Vector2::ZERO, swapchain_size);
        cmd_buffer.cmd_set_viewport(0, &[viewport]);
        cmd_buffer.cmd_set_scissor(0, &[scissor]);

        for &material in render_data.unlit_opaque_renderables() {
            let material = &render_objects.unlit_opaque_material(material);
            cmd_buffer.cmd_bind_pipeline(VulkanPipelineBindPoint::Graphics, material.pipeline());
            cmd_buffer.cmd_draw(3, 1, 0, 0);
        }
    }
}
