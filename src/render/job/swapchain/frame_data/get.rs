use crate::render::job::swapchain::FrameData;
use alexandria::gpu::VulkanSemaphore;

impl FrameData {
    /// Get the semaphore that signals when the swapchain image has been acquired and the GPU can
    /// begin rendering this frame
    pub fn acquire_image_semaphore(&mut self) -> &mut VulkanSemaphore {
        &mut self.acquire_image_semaphore
    }

    /// Get the semaphore that signals when rendering is complete for this frame
    pub fn render_complete_semaphore(&self) -> &VulkanSemaphore {
        &self.render_complete_semaphore
    }
}
