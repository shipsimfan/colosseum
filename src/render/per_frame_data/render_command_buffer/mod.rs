use crate::render::MAX_FRAMES_IN_FLIGHT;
use alexandria::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanCommandPool},
};

mod index;
mod new;

/// The render command buffer used for each frame in flight
pub(in crate::render) struct RenderCommandBuffer {
    /// The underlying Vulkan command pool
    command_pool: VulkanCommandPool,

    /// The command buffers for each frame in flight
    command_buffers: [Id<VulkanCommandBuffer>; MAX_FRAMES_IN_FLIGHT],
}
