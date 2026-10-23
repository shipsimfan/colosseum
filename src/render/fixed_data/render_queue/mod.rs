use alexandria::gpu::VulkanQueue;

mod deref;
mod new;

/// The Vulkan queue for submitting render commands
pub(in crate::render) struct RenderQueue {
    /// The underlying queue
    queue: VulkanQueue,
}
