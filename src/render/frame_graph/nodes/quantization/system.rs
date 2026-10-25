use crate::render::{
    FrameIndex, Pipeline, SecondaryCommandBuffer,
    frame_graph::{FrameGraphResources, QuantizationNode, nodes::quantization::PushConstants},
};
use alexandria::{
    gpu::{VulkanPipelineBindPoint, VulkanShaderStageFlag},
    math::Vector2f,
};
use colosseum_core::as_bytes;
use colosseum_ecs::{Res, ResMut};

impl QuantizationNode {
    /// Sharpen, apply dithering, and quantize an image
    pub(in crate::render::frame_graph::nodes::quantization) fn system(
        self: Res<Self>,
        pipeline: Res<Pipeline<QuantizationNode>>,
        frame_index: Res<FrameIndex>,
        resources: Res<FrameGraphResources>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        let cmd_buffer = &mut command_buffer[&frame_index];

        // Bind the pipeline for the quantization pass
        pipeline.bind(cmd_buffer);

        // Bind the descriptor set for the input image
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline.layout(),
            0,
            resources.descriptor_set(self.descriptor_set, &frame_index),
        );

        // Bind the push constants for the clear color
        let size = resources.get(self.output, &frame_index).size().into_f32();
        let push_constants = PushConstants {
            image_size: size,
            texel_size: Vector2f::new(1.0 / size.x, 1.0 / size.y),
            sharpness: 0.0,
        };
        cmd_buffer.cmd_push_constants(
            pipeline.layout(),
            VulkanShaderStageFlag::Fragment,
            0,
            as_bytes(&push_constants),
        );

        // Perform the draw
        cmd_buffer.cmd_draw(3, 1, 0, 0);
    }
}
