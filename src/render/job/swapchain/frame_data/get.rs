use crate::render::job::swapchain::FrameData;
use alexandria::gpu::{VulkanFence, VulkanSemaphore};

impl FrameData {
    /// Get all the semaphores and the fence for this frame, which are needed for submitting the command buffer
    pub fn semaphores_and_fence(
        &mut self,
    ) -> (&mut VulkanSemaphore, &VulkanSemaphore, &mut VulkanFence) {
        (
            &mut self.acquire_image_semaphore,
            &self.render_complete_semaphore,
            &mut self.draw_fence,
        )
    }
}
