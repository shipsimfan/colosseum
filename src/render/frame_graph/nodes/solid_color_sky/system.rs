use crate::render::{
    FrameIndex, Pipeline, SecondaryCommandBuffer, Skybox, as_bytes,
    frame_graph::{FrameGraphResources, SolidColorSkyNode},
};
use alexandria::{
    gpu::{
        VulkanAttachmentLoadOp, VulkanAttachmentStoreOp, VulkanImageLayout,
        VulkanRenderingAttachmentInfo, VulkanResolveModeFlag, VulkanShaderStageFlag,
        VulkanViewport,
    },
    math::{Color4f, Linear, Recti, Vector2, Vector2i},
};
use colosseum_ecs::{Res, ResMut};

impl SolidColorSkyNode {
    /// Clear the sky with a solid color
    pub(in crate::render::frame_graph::nodes::solid_color_sky) fn system(
        self: Res<Self>,
        skybox: Res<Skybox>,
        pipeline: Res<Pipeline<SolidColorSkyNode>>,
        resources: Res<FrameGraphResources>,
        frame_index: Res<FrameIndex>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        let cmd_buffer = &mut command_buffer[&frame_index];
        let output = resources.get(self.output);

        // Begin the render pass
        let size = output.size();
        cmd_buffer.cmd_begin_rendering(
            0,
            Vector2i::ZERO,
            size,
            1,
            0,
            &[VulkanRenderingAttachmentInfo::new(
                output.image_view(),
                VulkanImageLayout::ColorAttachmentOptimal,
                VulkanResolveModeFlag::None,
                None,
                VulkanImageLayout::Undefined,
                VulkanAttachmentLoadOp::Clear,
                VulkanAttachmentStoreOp::Store,
                Color4f::<Linear>::new(0.0, 0.0, 0.0, 1.0),
            )],
            None,
            None,
        );

        // Bind the viewport and scissor for the render pass
        let viewport = VulkanViewport::new(0.0, 0.0, size.x as _, size.y as _, 0.0, 1.0);
        let scissor = Recti::new(Vector2::ZERO, size);
        cmd_buffer.cmd_set_viewport(0, &[viewport]);
        cmd_buffer.cmd_set_scissor(0, &[scissor]);

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

        // End the render pass
        cmd_buffer.cmd_end_rendering();
    }
}
