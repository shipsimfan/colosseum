use alexandria::gpu::{VulkanFence, VulkanSemaphore};

mod get;
mod new;
mod wait_for_draw_finish;

/// The per-frame data for the swapchain
pub(in crate::render::job::swapchain) struct FrameData {
    /// Semaphore signaled when the swapchain image has been acquired and the GPU can begin
    /// rendering this frame
    acquire_image_semaphore: VulkanSemaphore,

    /// Semaphore signaled when the GPU has finished rendering this frame and presentation can
    /// begin
    render_complete_semaphore: VulkanSemaphore,

    /// Fence signaled when the GPU has finished executing the command buffer for this frame
    draw_fence: VulkanFence,
}
