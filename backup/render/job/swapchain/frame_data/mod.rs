use crate::render::FrameGraphTransientBuffer;
use alexandria::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanFence, VulkanSemaphore},
};

mod new;
mod wait_for_draw_finish;

/// The per-frame data for the swapchain
pub(in crate::render::job::swapchain) struct FrameData {
    /// The command buffer used for recording copy commands
    pub copy_command_buffer: Id<VulkanCommandBuffer>,

    /// The command buffer used for recording rendering commands
    pub render_command_buffer: Id<VulkanCommandBuffer>,

    /// Semaphore signaled when the swapchain image has been acquired and the GPU can begin
    /// rendering this frame
    pub acquire_image_semaphore: VulkanSemaphore,

    /// Semaphore signalled when the copy commands for this frame have completed
    pub copy_complete_semaphore: VulkanSemaphore,

    /// Semaphore signaled when the GPU has finished rendering this frame and presentation can
    /// begin
    pub render_complete_semaphore: VulkanSemaphore,

    /// Fence signaled when the GPU has finished executing the command buffer for this frame
    pub draw_fence: VulkanFence,

    /// The transient buffers that have been allocated for temporary data during rendering
    pub transient_buffer: FrameGraphTransientBuffer,
}
