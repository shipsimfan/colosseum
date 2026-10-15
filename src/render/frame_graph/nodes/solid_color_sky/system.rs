use crate::render::{
    FrameIndex, Pipeline, RenderCommandBuffer, Skybox, Swapchain, as_bytes,
    frame_graph::SolidColorSkyNode,
};
use alexandria::{
    gpu::{
        VK_QUEUE_FAMILY_IGNORED, VulkanAccessFlag, VulkanAttachmentLoadOp, VulkanAttachmentStoreOp,
        VulkanImageAspectFlag, VulkanImageLayout, VulkanImageMemoryBarrier,
        VulkanPipelineStageFlag, VulkanRenderingAttachmentInfo, VulkanResolveModeFlag,
        VulkanShaderStageFlag, VulkanViewport,
    },
    math::{Color4f, Linear, Recti, Vector2, Vector2i},
};
use colosseum_ecs::{Res, ResMut};

impl SolidColorSkyNode {
    /// Clear the sky with a solid color
    pub fn system(
        skybox: Res<Skybox>,
        pipeline: Res<Pipeline<SolidColorSkyNode>>,
        swapchain: Res<Swapchain>,
        frame_index: Res<FrameIndex>,
        mut command_buffer: ResMut<RenderCommandBuffer>,
    ) {
        let cmd_buffer = &mut command_buffer[&frame_index];

        // Insert the pipeline barrier
        cmd_buffer.cmd_pipeline_barrier2(
            0,
            &[],
            &[],
            &[VulkanImageMemoryBarrier::new(
                0,
                0,
                VulkanPipelineStageFlag::ColorAttachmentOutput,
                VulkanAccessFlag::ColorAttachmentWrite,
                VulkanImageLayout::Undefined,
                VulkanImageLayout::ColorAttachmentOptimal,
                VK_QUEUE_FAMILY_IGNORED,
                VK_QUEUE_FAMILY_IGNORED,
                swapchain.image(),
                VulkanImageAspectFlag::Color,
                0,
                1,
                0,
                1,
            )],
        );

        // Begin the render pass
        let size = swapchain.size();
        cmd_buffer.cmd_begin_rendering(
            0,
            Vector2i::ZERO,
            size,
            1,
            0,
            &[VulkanRenderingAttachmentInfo::new(
                swapchain.image_view(),
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
