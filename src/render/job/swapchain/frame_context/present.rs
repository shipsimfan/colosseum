use crate::{Error, Result, render::FrameContext};

impl<'frame, 'surface> FrameContext<'frame, 'surface> {
    /// Submit the command buffer for this frame and present the image to the swapchain
    pub fn present(mut self) -> Result<()> {
        // Transition the swapchain image to the present layout
        self.data.cmd_pipeline_barrier2(
            &self.swapchain.images()[self.image_index as usize],
            alexandria::gpu::VulkanImageLayout::ColorAttachmentOptimal,
            alexandria::gpu::VulkanImageLayout::PresentSrcKhr,
            alexandria::gpu::VulkanAccessFlag::ColorAttachmentWrite,
            alexandria::gpu::VulkanAccessFlags::default(),
            alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
            alexandria::gpu::VulkanPipelineStageFlag::BottomOfPipe,
        );

        // End the command buffer
        self.data.end().map_err(Error::new_inner)?;

        // Submit the command buffer
        self.data.submit(&mut self.queue)?;

        // Submit the present command
        self.queue
            .present(
                &self.data.render_complete_semaphore(),
                &self.swapchain,
                self.image_index,
            )
            .map_err(Error::new_inner)
    }
}
