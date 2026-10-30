use crate::{
    Result,
    render::{
        AcquireImageSemaphore, CopyCompleteSemaphore, DrawFence, FrameIndex, RenderCommandBuffer,
        RenderCompleteSemaphore, RenderQueue, Swapchain,
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
        frame_index: Res<FrameIndex>,
        mut command_buffer: ResMut<RenderCommandBuffer>,
        acquire_image_semaphore: Res<AcquireImageSemaphore>,
        copy_complete_semaphore: Res<CopyCompleteSemaphore>,
        render_complete_semaphore: Res<RenderCompleteSemaphore>,
        mut render_queue: ResMut<RenderQueue>,
        mut draw_fence: ResMut<DrawFence>,
    ) -> Result<()> {
        let command_buffer = &mut command_buffer[&frame_index];

        render_queue
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[
                        VulkanSemaphoreSubmitInfo::new(
                            &acquire_image_semaphore[&frame_index],
                            0,
                            VulkanPipelineStageFlag::ColorAttachmentOutput,
                            0,
                        ),
                        VulkanSemaphoreSubmitInfo::new(
                            &copy_complete_semaphore[&frame_index],
                            0,
                            VulkanPipelineStageFlag::VertexShader,
                            0,
                        ),
                    ],
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
        render_queue
            .present(
                Some(&render_complete_semaphore[&frame_index]),
                &self.swapchain,
                self.image_index as _,
            )
            .map_err(|error| new_error!("unable to queue a swapchain present - {}", error))
    }
}
