use crate::render::{
    FixedRenderObjects, RenderData, RenderObjects,
    frame_graph::{FrameGraphResources, GammaCorrectionNode},
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanViewport},
    math::{Recti, Vector2},
};

impl GammaCorrectionNode {
    /// Execute the solid color sky pass, rendering a full-screen quad with the specified clear color
    pub(in crate::render::frame_graph) fn execute(
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
        let pipeline = render_objects.pipeline(FixedRenderObjects::GAMMA_CORRECTION_PIPELINE);
        pipeline.bind(cmd_buffer);

        // Bind the descriptor set for the input image
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline.layout(),
            0,
            render_data.post_process_descriptor_set(RenderData::GAMMA_CORRECTION_DESCRIPTOR_SET),
        );

        // Perform the draw
        cmd_buffer.cmd_draw(3, 1, 0, 0);
    }
}
