use crate::render::{
    FrameIndex, Pipeline, SecondaryCommandBuffer, Skybox, as_bytes, frame_graph::SolidColorSkyNode,
};
use alexandria::gpu::VulkanShaderStageFlag;
use colosseum_ecs::{Res, ResMut};

impl SolidColorSkyNode {
    /// Clear the sky with a solid color
    pub(in crate::render::frame_graph::nodes::solid_color_sky) fn system(
        skybox: Res<Skybox>,
        pipeline: Res<Pipeline<SolidColorSkyNode>>,
        frame_index: Res<FrameIndex>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        let cmd_buffer = &mut command_buffer[&frame_index];

        // Bind the pipeline for the solid color sky pass
        pipeline.bind(cmd_buffer);

        // Bind the push constants for the clear color
        let clear_color = match skybox.as_ref() {
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
