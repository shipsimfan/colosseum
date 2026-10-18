use crate::render::MAX_FRAMES_IN_FLIGHT;
use alexandria::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanCommandPool},
};
use std::marker::PhantomData;

mod index;
mod new;

/// A wrapper around a secondary Vulkan command buffer used by nodes in the frame graph
pub(in crate::render) struct SecondaryCommandBuffer<Node: 'static> {
    /// The underlying Vulkan command pool
    command_pool: VulkanCommandPool,

    /// The command buffers for each frame in flight
    command_buffers: [Id<VulkanCommandBuffer>; MAX_FRAMES_IN_FLIGHT],

    /// A marker for the node this secondary command buffer is associated with
    _node: PhantomData<Node>,
}
