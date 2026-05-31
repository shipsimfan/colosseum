use crate::render::job::swapchain::FrameData;
use alexandria::gpu::VulkanSemaphore;

impl FrameData {
    /// Get the semaphore that signals when rendering is complete for this frame
    pub fn render_complete_semaphore(&self) -> &VulkanSemaphore {
        &self.render_complete_semaphore
    }

    /// Get the semaphore that signals when presentation is complete for this frame
    pub fn present_complete_semaphore(&self) -> &VulkanSemaphore {
        &self.present_complete_semaphore
    }
}
