use crate::{
    Result,
    render::{
        AcquireImageSemaphore, DrawFence, FrameIndex, GraphicsDevice, RenderCommandBuffer,
        RenderCompleteSemaphore, Swapchain,
    },
};
use alexandria::gpu::{
    VK_QUEUE_FAMILY_IGNORED, VulkanAccessFlag, VulkanCommandBufferSubmitInfo,
    VulkanImageAspectFlag, VulkanImageLayout, VulkanImageMemoryBarrier, VulkanPipelineStageFlag,
    VulkanSemaphoreSubmitInfo, VulkanSubmitInfo,
};
use colosseum_core::new_error;
use colosseum_ecs::{Res, ResMut};

impl Swapchain {
    /// Present the currently acquired swapchain image
    pub fn present(
        self: ResMut<Self>,
        mut device: ResMut<GraphicsDevice>,
        frame_index: Res<FrameIndex>,
        mut command_buffer: ResMut<RenderCommandBuffer>,
        acquire_image_semaphore: Res<AcquireImageSemaphore>,
        render_complete_semaphore: Res<RenderCompleteSemaphore>,
        mut draw_fence: ResMut<DrawFence>,
    ) -> Result<()> {
        let command_buffer = &mut command_buffer[&frame_index];

        // Insert a final pipeline barrier to prepare for present
        command_buffer.cmd_pipeline_barrier2(
            0,
            &[],
            &[],
            &[VulkanImageMemoryBarrier::new(
                VulkanPipelineStageFlag::ColorAttachmentOutput,
                VulkanAccessFlag::ColorAttachmentWrite,
                VulkanPipelineStageFlag::ColorAttachmentOutput,
                VulkanAccessFlag::ColorAttachmentWrite,
                VulkanImageLayout::ColorAttachmentOptimal,
                VulkanImageLayout::PresentSrcKhr,
                VK_QUEUE_FAMILY_IGNORED,
                VK_QUEUE_FAMILY_IGNORED,
                self.image(),
                VulkanImageAspectFlag::Color,
                0,
                1,
                0,
                1,
            )],
        );

        // End and submit the command buffer
        command_buffer
            .end()
            .map_err(|error| new_error!("unable to end render command buffer - {}", error))?;
        device
            .render_queue()
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[VulkanSemaphoreSubmitInfo::new(
                        &acquire_image_semaphore[&frame_index],
                        0,
                        VulkanPipelineStageFlag::ColorAttachmentOutput,
                        0,
                    )],
                    &[VulkanCommandBufferSubmitInfo::new(command_buffer, 0)],
                    &[VulkanSemaphoreSubmitInfo::new(
                        &render_complete_semaphore[&frame_index],
                        0,
                        VulkanPipelineStageFlag::ColorAttachmentOutput,
                        0,
                    )],
                )],
                Some(&mut draw_fence.get_mut(&frame_index)),
            )
            .map_err(|error| {
                new_error!(
                    "unable to submit render command buffer to queue - {}",
                    error
                )
            })?;

        // Submit the present
        device
            .render_queue()
            .present(
                Some(&render_complete_semaphore[&frame_index]),
                &self.swapchain,
                self.image_index as _,
            )
            .map_err(|error| new_error!("unable to queue a swapchain present - {}", error))
    }
}
