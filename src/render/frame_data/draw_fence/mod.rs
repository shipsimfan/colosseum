use crate::render::MAX_FRAMES_IN_FLIGHT;
use alexandria::gpu::VulkanFence;

mod index;
mod new;

/// A wrapper around a Vulkan fence used for synchronizing draw operations
pub(in crate::render) struct DrawFence {
    /// The underlying Vulkan fence handles
    fences: [VulkanFence; MAX_FRAMES_IN_FLIGHT],
}
