use crate::render::{
    FixedRenderObjects, RenderData, RenderObjects, as_bytes,
    frame_graph::{FrameGraphResources, QuantizationNode, nodes::quantization::PushConstants},
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanShaderStageFlag, VulkanViewport},
    math::{Recti, Vector2},
};

impl QuantizationNode {
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
        let pipeline = render_objects.pipeline(FixedRenderObjects::QUANTIZATION_PIPELINE);
        pipeline.bind(cmd_buffer);

        // Bind the descriptor set for the input image
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline.layout(),
            0,
            render_data.post_process_descriptor_set(RenderData::QUANTIZATION_DESCRIPTOR_SET),
        );

        // Push texel size
        let size = size.into_f32();
        let push_constants = PushConstants {
            image_size: size,
            texel_size: 1.0 / size,
            sharpness: if render_data.render_scale() == 1.0 {
                0.0
            } else if render_data.render_scale() < 1.0 {
                -0.2
            } else {
                -0.125
            },
        };
        cmd_buffer.cmd_push_constants(
            pipeline.layout(),
            VulkanShaderStageFlag::Fragment,
            0,
            unsafe { as_bytes(&push_constants) },
        );

        // Perform the draw
        cmd_buffer.cmd_draw(3, 1, 0, 0);
    }
}
