use crate::render::{CopyFence, FrameIndex};
use alexandria::gpu::VulkanFence;

impl CopyFence {
    /// Get a mutable reference to a fence for submitting
    pub fn get_mut(&mut self, frame_index: &FrameIndex) -> &mut VulkanFence {
        &mut self.fences[**frame_index]
    }
}
