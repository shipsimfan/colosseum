use crate::{
    Result, Window,
    render::{AcquireImageSemaphore, FrameIndex, Swapchain},
};
use colosseum_core::{debug, new_error};
use colosseum_ecs::{Res, ResMut};

impl Swapchain {
    /// Acquires the next image from the swapchain for rendering
    pub fn acquire_next_image(
        mut self: ResMut<Self>,
        window: Res<Window>,
        frame_index: Res<FrameIndex>,
        mut acquire_image_semaphore: ResMut<AcquireImageSemaphore>,
    ) -> Result<()> {
        // Check if the window size has changed and handle swapchain recreation if necessary
        if self.size != window.size() {
            self.valid = false;
            return Ok(());
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
                self.valid = false;
                return Ok(());
            }
        };

        self.suboptimal = suboptimal;
        self.image_index = image_index;

        Ok(())
    }
}
