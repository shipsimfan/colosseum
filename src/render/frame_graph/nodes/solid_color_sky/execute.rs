use crate::render::{RenderData, RenderObjects, Skybox, frame_graph::SolidColorSkyNode};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanShaderStageFlag, VulkanViewport},
    math::{Color4f, Linear, Recti, Vector2, Vector2u},
};

impl SolidColorSkyNode {
    /// Execute the solid color sky pass, rendering a full-screen quad with the specified clear color
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

        // Bind the pipeline for the solid color sky pass
        render_objects.pipelines()[0].bind(cmd_buffer);

        // Bind the push constants for the clear color
        let clear_color = match render_data.skybox() {
            Skybox::SolidColor(color) => color.with_alpha(1.0),
        };
        cmd_buffer.cmd_push_constants(
            render_objects.pipelines()[0].layout(),
            VulkanShaderStageFlag::Fragment,
            0,
            unsafe {
                std::slice::from_raw_parts(
                    (&clear_color as *const Color4f<Linear>).cast(),
                    std::mem::size_of::<Color4f<Linear>>(),
                )
            },
        );

        // Perform the draw
        cmd_buffer.cmd_draw(3, 1, 0, 0);
    }
}
