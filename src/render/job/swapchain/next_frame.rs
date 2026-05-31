use crate::{
    Error, Result,
    render::{FrameContext, job::Swapchain},
};
use alexandria::math::Vector2u;

impl<'surface> Swapchain<'surface> {
    /// Get the context for the next frame, or [`None`] if the swapchain is out of date and needs
    /// to be recreated
    pub fn next_frame<'frame>(
        &'frame mut self,
        size: Vector2u,
    ) -> Result<Option<FrameContext<'frame, 'surface>>> {
        // Get the next frame data
        let frame = &mut self.frame_data[self.frame_index];
        self.frame_index = (self.frame_index + 1) % self.image_views.len();

        // Wait for the previous frame to finish
        frame.wait_for_draw_finish()?;

        if self.size != size {
            return Ok(None);
        }

        // Acquire the next image to render into
        let image_index = match self
            .swapchain
            .acquire_next_image(u64::MAX, frame.present_complete_semaphore())
            .unwrap()
        {
            Some(image_index) => image_index,
            None => return Ok(None),
        };

        // Begin the command buffer for the frame
        frame.begin().map_err(Error::new_inner)?;

        // Transition the swapchain image to the color attachment layout
        frame.cmd_pipeline_barrier2(
            &self.swapchain.images()[image_index],
            alexandria::gpu::VulkanImageLayout::Undefined,
            alexandria::gpu::VulkanImageLayout::ColorAttachmentOptimal,
            alexandria::gpu::VulkanAccessFlags::default(),
            alexandria::gpu::VulkanAccessFlag::ColorAttachmentWrite,
            alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
            alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
        );

        Ok(Some(FrameContext::new(
            frame,
            self.device.queue(),
            image_index as _,
            &mut self.image_views[image_index],
            &mut self.swapchain,
            self.size,
        )))
    }
}
