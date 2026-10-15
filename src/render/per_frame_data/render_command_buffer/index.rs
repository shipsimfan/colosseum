use crate::render::{FrameIndex, RenderCommandBuffer};
use alexandria::gpu::VulkanCommandBuffer;
use std::ops::{Index, IndexMut};

impl Index<&FrameIndex> for RenderCommandBuffer {
    type Output = VulkanCommandBuffer;

    fn index(&self, index: &FrameIndex) -> &Self::Output {
        &self.command_pool[self.command_buffers[**index]]
    }
}

impl IndexMut<&FrameIndex> for RenderCommandBuffer {
    fn index_mut(&mut self, index: &FrameIndex) -> &mut Self::Output {
        &mut self.command_pool[self.command_buffers[**index]]
    }
}
