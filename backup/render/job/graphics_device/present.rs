use crate::{Error, Result, render::job::GraphicsDevice};
use alexandria::gpu::{VulkanSemaphore, VulkanSwapchain};

impl GraphicsDevice {
    /// Present the rendered image to the swapchain
    pub fn present(
        &mut self,
        render_complete_semaphore: &VulkanSemaphore,
        swapchain: &VulkanSwapchain,
        image_index: u32,
    ) -> Result<()> {
        self.queue
            .present(Some(render_complete_semaphore), swapchain, image_index)
            .map_err(Error::new_inner)
    }
}
