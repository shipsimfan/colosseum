use crate::{
    SettingsCache,
    render::{
        ColorCorrection, FrameIndex, Pipeline, SecondaryCommandBuffer, as_bytes,
        frame_graph::{FrameGraphResources, ToneMapNode, nodes::tone_map::PushConstants},
    },
};
use alexandria::gpu::{VulkanPipelineBindPoint, VulkanShaderStageFlag};
use colosseum_ecs::{Res, ResMut};

impl<Game: crate::Game> ToneMapNode<Game> {
    /// Performs tone mapping on the input image
    pub(in crate::render::frame_graph::nodes::tone_map) fn system(
        self: Res<Self>,
        settings: Res<Game::SettingsCache>,
        color_correction: Res<ColorCorrection>,
        pipeline: Res<Pipeline<ToneMapNode<Game>>>,
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
        let push_constants = PushConstants {
            contrast: color_correction.contrast,
            saturation: color_correction.saturation,
            exposure: color_correction.exposure,
            gamma: settings.display_settings().gamma(),
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
