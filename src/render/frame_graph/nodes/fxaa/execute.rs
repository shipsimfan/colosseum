use crate::render::{
    FixedRenderObjects, RenderData, RenderObjects, as_bytes,
    frame_graph::{FrameGraphResources, FxaaNode},
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanShaderStageFlag, VulkanViewport},
    math::{Recti, Vector2},
};

impl FxaaNode {
    /// Execute the FXAA pass, rendering a full-screen quad with anti-aliasing
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

        // Bind the pipeline for the FXAA pass
        let pipeline = render_objects.pipeline(FixedRenderObjects::FXAA_PIPELINE);
        pipeline.bind(cmd_buffer);

        // Bind the descriptor set for the input image
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline.layout(),
            0,
            render_data.post_process_descriptor_set(RenderData::FXAA_DESCRIPTOR_SET),
        );

        // Push texel size
        let texel_size = 1.0 / size.into_f32();
        cmd_buffer.cmd_push_constants(
            pipeline.layout(),
            VulkanShaderStageFlag::Fragment,
            0,
            unsafe { as_bytes(&texel_size) },
        );

        // Perform the draw
        cmd_buffer.cmd_draw(3, 1, 0, 0);
    }
}
