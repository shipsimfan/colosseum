use crate::render::MAX_FRAMES_IN_FLIGHT;
use alexandria::gpu::VulkanFence;

mod get;
mod new;
mod wait;

/// A wrapper around a Vulkan fence used for synchronizing draw operations
pub(in crate::render) struct DrawFence {
    /// The underlying Vulkan fence handles and if they were submitted
    fences: [(VulkanFence, bool); MAX_FRAMES_IN_FLIGHT],
}
