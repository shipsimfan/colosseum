use crate::render::{DrawFence, FrameIndex};
use alexandria::gpu::VulkanFence;

impl DrawFence {
    /// Get a mutable reference to a fence for submitting
    pub fn get_mut(&mut self, frame_index: &FrameIndex) -> &mut VulkanFence {
        let (fence, signalled) = &mut self.fences[**frame_index];
        *signalled = true;
        fence
    }
}
