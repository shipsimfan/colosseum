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

        // Check if the swapchain is out of date and needs to be recreated
        if self.size != size {
            return Ok(true);
        }

        // Copy all required data to the GPU (renderables, lights, and camera data)
        let copy_cmd_buffer = &mut self.command_pool[frame.copy_command_buffer];
        device.copy_data(
            &token,
            &mut frame.transient_buffer,
            &frame.copy_complete_semaphore,
            copy_cmd_buffer,
        )?;

        // Acquire the next image to render into
        let image_index = match self
            .swapchain
            .as_mut()
            .unwrap()
            .acquire_next_image(u64::MAX, Some(&mut frame.acquire_image_semaphore), None, 1)
            .map_err(Error::new_inner)?
        {
            Some(image_index) => image_index,
            None => return Ok(true),
        };

        // Begin the command buffer for rendering the frame
        let render_cmd_buffer = &mut self.command_pool[frame.render_command_buffer];
        render_cmd_buffer.begin().map_err(Error::new_inner)?;

        // Build and execute the frame graph for this frame
        device.build_and_run_frame_graph(
            &token,
            self.size,
            &self.image_views[image_index as usize],
            &mut frame.transient_buffer,
            render_cmd_buffer,
        )?;

        // End the rendering command buffer
        render_cmd_buffer.end().map_err(Error::new_inner)?;

        // Submit the render command buffer for execution
        device.submit_render(
            &frame.acquire_image_semaphore,
            &frame.copy_complete_semaphore,
            &frame.render_complete_semaphore,
            &mut frame.draw_fence,
            render_cmd_buffer,
        )?;

        // Present the rendered image
        device.present(
            &frame.render_complete_semaphore,
            self.swapchain.as_ref().unwrap(),
            image_index as _,
        )?;

        Ok(false)
    }
}
