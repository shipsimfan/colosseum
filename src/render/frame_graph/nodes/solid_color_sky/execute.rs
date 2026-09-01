use crate::render::{
    FixedRenderObjects, RenderData, RenderObjects, Skybox, as_bytes,
    frame_graph::{FrameGraphResources, SolidColorSkyNode},
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanShaderStageFlag, VulkanViewport},
    math::{Recti, Vector2},
};

impl SolidColorSkyNode {
    /// Execute the solid color sky pass, rendering a full-screen quad with the specified clear color
    pub(in crate::render::frame_graph::nodes) fn execute(
        &self,
        render_data: &RenderData,
        render_objects: &RenderObjects,
        resources: &FrameGraphResources,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        let size = resources.get(self.output).size();

        // Bind the viewport and scissor for the render pass
        let viewport = VulkanViewport::new(0.0, 0.0, size.x as _, size.y as _, 0.0, 1.0);
        let scissor = Recti::new(Vector2::ZERO, size);
        cmd_buffer.cmd_set_viewport(0, &[viewport]);
        cmd_buffer.cmd_set_scissor(0, &[scissor]);

        // Bind the pipeline for the solid color sky pass
        let pipeline = render_objects.pipeline(FixedRenderObjects::SOLID_COLOR_SKY_PIPELINE);
        pipeline.bind(cmd_buffer);

        // Bind the push constants for the clear color
        let clear_color = match render_data.skybox() {
            Skybox::SolidColor(color) => color.with_alpha(1.0),
        };
        cmd_buffer.cmd_push_constants(
            pipeline.layout(),
            VulkanShaderStageFlag::Fragment,
            0,
            unsafe { as_bytes(&clear_color) },
        );

        // Perform the draw
        cmd_buffer.cmd_draw(3, 1, 0, 0);
    }
}
