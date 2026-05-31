use alexandria::gpu::{VulkanCommandBuffer, VulkanFence, VulkanSemaphore};

mod deref;
mod get;
mod new;
mod submit;
mod wait_for_draw_finish;

/// The per-frame data for the swapchain
pub(in crate::render::job::swapchain) struct FrameData {
    /// The command buffer to record rendering commands into
    command_buffer: VulkanCommandBuffer,

    /// Semaphore signaled when the GPU has finished rendering this frame and presentation can begin
    render_complete_semaphore: VulkanSemaphore,

    /// Semaphore signaled when the presentation engine has finished reading from the swapchain image
    present_complete_semaphore: VulkanSemaphore,

    /// Fence signaled when the GPU has finished executing the command buffer for this frame
    draw_fence: VulkanFence,
}
