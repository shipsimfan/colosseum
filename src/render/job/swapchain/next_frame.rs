use crate::{
    Error, Result,
    render::{RenderData, job::Swapchain},
};
use alexandria::{
    gpu::{VulkanAccessFlag, VulkanAccessFlags, VulkanImageLayout, VulkanPipelineStageFlag},
    math::Vector2u,
};
use std::ops::DerefMut;

impl<'surface> Swapchain<'surface> {
    /// Get the context for the next frame, or true if the swapchain is out of date and needs
    /// to be recreated
    pub fn next_frame<'frame>(
        &'frame mut self,
        size: Vector2u,
        render_data: &RenderData,
    ) -> Result<bool> {
        // Get the next frame data
        let frame = &mut self.frame_data[self.frame_index];
        self.frame_index = (self.frame_index + 1) % self.image_views.len();

        // Wait for the previous frame to finish
        frame.wait_for_draw_finish()?;

        // Check if the swapchain is out of date and needs to be recreated
        if self.size != size {
            return Ok(true);
        }

        // Begin the command buffer for the frame
        frame.begin().map_err(Error::new_inner)?;

        // Acquire the next image to render into
        let image_index = match self
            .swapchain
            .acquire_next_image(u64::MAX, frame.present_complete_semaphore())
            .unwrap()
        {
            Some(image_index) => image_index,
            None => return Ok(true),
        };

        // Build and execute the frame graph for this frame
        self.device.build_and_run_frame_graph(
            render_data,
            &self.image_views[image_index as usize],
            self.size,
            self.device.swapchain_format(),
            frame.deref_mut(),
        );

        // Transition the swapchain image to the present layout
        frame.cmd_pipeline_barrier2(
            &self.swapchain.images()[image_index],
            VulkanImageLayout::ColorAttachmentOptimal,
            VulkanImageLayout::PresentSrcKhr,
            VulkanAccessFlag::ColorAttachmentWrite,
            VulkanAccessFlags::default(),
            VulkanPipelineStageFlag::ColorAttachmentOutput,
            VulkanPipelineStageFlag::BottomOfPipe,
        );

        // End the command buffer
        frame.end().map_err(Error::new_inner)?;

        // Submit the command buffer for execution
        frame.submit(self.device.queue())?;

        // Present the rendered image
        self.device
            .queue()
            .present(
                &frame.render_complete_semaphore(),
                &self.swapchain,
                image_index as _,
            )
            .map_err(Error::new_inner)?;

        Ok(false)
    }
}
