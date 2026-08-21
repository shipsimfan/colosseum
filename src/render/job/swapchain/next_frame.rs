use crate::{
    Error, Result,
    render::job::{GraphicsDevice, Swapchain},
};
use alexandria::math::Vector2u;

impl<'surface> Swapchain<'surface> {
    /// Get the context for the next frame, or true if the swapchain is out of date and needs
    /// to be recreated
    pub fn next_frame<'frame>(
        &'frame mut self,
        size: Vector2u,
        device: &mut GraphicsDevice,
    ) -> Result<bool> {
        // Get the next frame data
        let frame_index = self.frame_index;
        let frame = &mut self.frame_data[frame_index];
        self.frame_index = (frame_index + 1) % self.image_views.len();

        // Apply any changes to the graphics device that have been queued up
        let token = device.apply_changes()?;

        // Wait for the previous frame to finish
        frame.wait_for_draw_finish()?;
        let (acquire_image_semaphore, render_complete_semaphore, draw_fence) =
            frame.semaphores_and_fence();

        // Check if the swapchain is out of date and needs to be recreated
        if self.size != size {
            return Ok(true);
        }

        // Acquire the next image to render into
        let image_index = match self
            .swapchain
            .acquire_next_image(u64::MAX, Some(acquire_image_semaphore), None, 1)
            .unwrap()
        {
            Some(image_index) => image_index,
            None => return Ok(true),
        };

        // Begin the command buffer for the frame
        device
            .command_buffer(frame_index)
            .begin()
            .map_err(Error::new_inner)?;

        // Build and execute the frame graph for this frame
        device.build_and_run_frame_graph(
            token,
            self.size,
            &self.image_views[image_index as usize],
            frame_index,
        )?;

        // End the command buffer
        device
            .command_buffer(frame_index)
            .end()
            .map_err(Error::new_inner)?;

        // Submit the command buffer for execution
        device.submit(
            frame_index,
            acquire_image_semaphore,
            render_complete_semaphore,
            draw_fence,
        )?;

        // Present the rendered image
        device.present(render_complete_semaphore, &self.swapchain, image_index as _)?;

        Ok(false)
    }
}
