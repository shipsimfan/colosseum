use crate::{
    Result,
    render::{
        AcquireImageSemaphore, DrawFence, FrameIndex, GraphicsDevice, RenderCommandBuffer,
        RenderCompleteSemaphore, Swapchain,
    },
};
use alexandria::gpu::{
    VulkanCommandBufferSubmitInfo, VulkanPipelineStageFlag, VulkanSemaphoreSubmitInfo,
    VulkanSubmitInfo,
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
