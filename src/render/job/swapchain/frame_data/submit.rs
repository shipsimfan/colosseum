use crate::{Error, Result, render::job::swapchain::FrameData};
use alexandria::gpu::{
    VulkanCommandBuffer, VulkanCommandBufferSubmitInfo, VulkanPipelineStageFlag, VulkanQueue,
    VulkanSemaphoreSubmitInfo, VulkanSubmitInfo,
};

impl FrameData {
    /// Submit the command buffer for this frame to the given queue, and signal the semaphores and
    /// fence
    pub fn submit(
        &mut self,
        queue: &mut VulkanQueue,
        command_buffer: &mut VulkanCommandBuffer,
    ) -> Result<()> {
        queue
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[VulkanSemaphoreSubmitInfo::new(
                        &self.acquire_image_semaphore,
                        0,
                        VulkanPipelineStageFlag::ColorAttachmentOutput,
                        0,
                    )],
                    &[VulkanCommandBufferSubmitInfo::new(&command_buffer, 0)],
                    &[VulkanSemaphoreSubmitInfo::new(
                        &self.render_complete_semaphore,
                        0,
                        VulkanPipelineStageFlag::ColorAttachmentOutput,
                        0,
                    )],
                )],
                Some(&mut self.draw_fence),
            )
            .map_err(Error::new_inner)
    }
}
