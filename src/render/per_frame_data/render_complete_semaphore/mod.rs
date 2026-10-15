use crate::render::MAX_FRAMES_IN_FLIGHT;
use alexandria::gpu::VulkanSemaphore;

mod index;
mod new;

/// A wrapper around a Vulkan semaphore used for synchronizing render completion
pub(in crate::render) struct RenderCompleteSemaphore {
    /// The underlying Vulkan semaphore handles
    semaphores: [VulkanSemaphore; MAX_FRAMES_IN_FLIGHT],
}
