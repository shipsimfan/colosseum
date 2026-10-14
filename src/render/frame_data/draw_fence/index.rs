use crate::render::{DrawFence, FrameIndex};
use alexandria::gpu::VulkanFence;
use std::ops::Index;

impl Index<&FrameIndex> for DrawFence {
    type Output = VulkanFence;

    fn index(&self, index: &FrameIndex) -> &Self::Output {
        &self.fences[**index]
    }
}
