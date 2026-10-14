use crate::{
    Result, Window,
    render::{AcquireImageSemaphore, DrawFence, FrameIndex, Render, Swapchain},
};
use colosseum_core::{debug, new_error};
use colosseum_ecs::{Res, ResMut, World};

impl Swapchain {
    /// Register the render frame system with the ECS
    pub fn register_render_frame_system(world: &mut World) {
        world.register_system::<Render, _, _>(Swapchain::render_frame);
    }

    /// Renders a single frame using the swapchain
    fn render_frame(
        mut self: ResMut<Swapchain>,
        frame_index: Res<FrameIndex>,
        draw_fence: Res<DrawFence>,
        mut acquire_image_semaphore: ResMut<AcquireImageSemaphore>,
        window: Res<Window>,
    ) -> Result<()> {
        // Wait for the previous frame to finish before rendering the next one
        draw_fence[&frame_index]
            .wait(u64::MAX)
            .map_err(|error| new_error!("unable to wait for draw fence - {}", error))?;

        // Check if the window size has changed and handle swapchain recreation if necessary
        if self.size != window.size() {
            todo!("remove swapchain for window resize");
        }

        // Acquire the next image from the swapchain
        let (image_index, suboptimal) = match self
            .swapchain
            .acquire_next_image(
                u64::MAX,
                Some(&mut acquire_image_semaphore[&frame_index]),
                None,
                1,
            )
            .map_err(|error| new_error!("unable to acquire next image - {}", error))?
        {
            Some(image_index) => image_index,
            None => {
                debug!(self.logger, "Swapchain is out of date");
                todo!("remove swapchain for out of date swapchain");
            }
        };

        // Begin the render command buffer

        // Perform a single clear render pass on the swapchain image

        // End the render command buffer

        // Submit the render command buffer to the graphics queue

        // Present the rendered image to the swapchain

        // Handle suboptimal swapchain if necessary
        if suboptimal {
            debug!(self.logger, "Swapchain is suboptimal");
            todo!("remove swapchain for suboptimal swapchain");
        }

        Ok(())
    }
}
