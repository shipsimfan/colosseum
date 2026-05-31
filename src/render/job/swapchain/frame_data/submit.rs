use crate::{Error, Result, render::job::swapchain::FrameData};
use alexandria::gpu::VulkanQueue;

impl FrameData {
    /// Submit the command buffer for this frame to the given queue, and signal the semaphores and
    /// fence
    pub fn submit(&mut self, queue: &mut VulkanQueue) -> Result<()> {
        queue
            .submit(
                &self.command_buffer,
                &self.present_complete_semaphore,
                &self.render_complete_semaphore,
                &mut self.draw_fence,
            )
            .map_err(Error::new_inner)
    }
}
