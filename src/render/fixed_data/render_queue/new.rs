use crate::render::RenderQueue;
use alexandria::gpu::VulkanQueue;

impl RenderQueue {
    /// Create a new [`RenderQueue`]
    pub fn new(queue: VulkanQueue) -> RenderQueue {
        RenderQueue { queue }
    }
}
