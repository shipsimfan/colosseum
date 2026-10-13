use crate::{Error, Result, render::job::GraphicsDevice};
use alexandria::gpu::{
    VulkanCommandBuffer, VulkanCommandBufferSubmitInfo, VulkanFence, VulkanPipelineStageFlag,
    VulkanSemaphore, VulkanSemaphoreSubmitInfo, VulkanSubmitInfo,
};

impl GraphicsDevice {
    /// Submit a command buffer with render commands to the graphics queue
    pub fn submit_render(
        &mut self,
        acquire_image_semaphore: &VulkanSemaphore,
        copy_complete_semaphore: &VulkanSemaphore,
        render_complete_semaphore: &VulkanSemaphore,
        draw_fence: &mut VulkanFence,
        cmd_buffer: &VulkanCommandBuffer,
    ) -> Result<()> {
        self.queue
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[
                        VulkanSemaphoreSubmitInfo::new(
                            acquire_image_semaphore,
                            0,
                            VulkanPipelineStageFlag::ColorAttachmentOutput,
                            0,
                        ),
                        VulkanSemaphoreSubmitInfo::new(
                            copy_complete_semaphore,
                            0,
                            VulkanPipelineStageFlag::VertexShader,
                            0,
                        ),
                    ],
                    &[VulkanCommandBufferSubmitInfo::new(cmd_buffer, 0)],
                    &[VulkanSemaphoreSubmitInfo::new(
                        &render_complete_semaphore,
                        0,
                        VulkanPipelineStageFlag::ColorAttachmentOutput,
                        0,
                    )],
                )],
                Some(draw_fence),
            )
            .map_err(Error::new_inner)
    }
}
