use crate::render::{
    FrameIndex, Pipeline, SecondaryCommandBuffer,
    frame_graph::{FrameGraphResources, FxaaNode},
};
use alexandria::gpu::{VulkanPipelineBindPoint, VulkanShaderStageFlag};
use colosseum_core::as_bytes;
use colosseum_ecs::{Res, ResMut};

impl FxaaNode {
    /// Performs tone mapping on the input image
    pub(in crate::render::frame_graph::nodes::fxaa) fn system(
        self: Res<Self>,
        pipeline: Res<Pipeline<FxaaNode>>,
        frame_index: Res<FrameIndex>,
        resources: Res<FrameGraphResources>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        let cmd_buffer = &mut command_buffer[&frame_index];

        // Bind the pipeline for the tone mapping pass
        pipeline.bind(cmd_buffer);

        // Bind the descriptor set for the input image
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline.layout(),
            0,
            resources.descriptor_set(self.descriptor_set, &frame_index),
        );

        // Bind the push constants for the tone mapping pass
        let texel_size = 1.0 / resources.get(self.output, &frame_index).size().into_f32();
        cmd_buffer.cmd_push_constants(
            pipeline.layout(),
            VulkanShaderStageFlag::Fragment,
            0,
            as_bytes(&texel_size),
        );

        // Perform the draw
        cmd_buffer.cmd_draw(3, 1, 0, 0);
    }
}
