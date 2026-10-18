use crate::render::{FrameIndex, SecondaryCommandBuffer};
use alexandria::gpu::VulkanCommandBuffer;
use std::ops::{Index, IndexMut};

impl<Node> Index<&FrameIndex> for SecondaryCommandBuffer<Node> {
    type Output = VulkanCommandBuffer;

    fn index(&self, index: &FrameIndex) -> &Self::Output {
        &self.command_pool[self.command_buffers[**index]]
    }
}

impl<Node> IndexMut<&FrameIndex> for SecondaryCommandBuffer<Node> {
    fn index_mut(&mut self, index: &FrameIndex) -> &mut Self::Output {
        &mut self.command_pool[self.command_buffers[**index]]
    }
}
