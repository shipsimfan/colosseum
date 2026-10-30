use crate::render::MAX_FRAMES_IN_FLIGHT;
use alexandria::gpu::VulkanFence;

mod get;
mod new;
mod wait;

/// A wrapper around a Vulkan fence used for synchronizing copy operations
pub(in crate::render) struct CopyFence {
    /// The underlying Vulkan fence handles
    fences: [VulkanFence; MAX_FRAMES_IN_FLIGHT],
}
