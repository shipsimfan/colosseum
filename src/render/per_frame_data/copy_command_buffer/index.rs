use crate::render::{CopyCommandBuffer, FrameIndex};
use alexandria::gpu::VulkanCommandBuffer;
use std::ops::{Index, IndexMut};

impl Index<&FrameIndex> for CopyCommandBuffer {
    type Output = VulkanCommandBuffer;

    fn index(&self, index: &FrameIndex) -> &Self::Output {
        &self.command_pool[self.command_buffers[**index]]
    }
}

impl IndexMut<&FrameIndex> for CopyCommandBuffer {
    fn index_mut(&mut self, index: &FrameIndex) -> &mut Self::Output {
        &mut self.command_pool[self.command_buffers[**index]]
    }
}
