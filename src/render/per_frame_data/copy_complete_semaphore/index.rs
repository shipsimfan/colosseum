use crate::render::{CopyCompleteSemaphore, FrameIndex};
use alexandria::gpu::VulkanSemaphore;
use std::ops::{Index, IndexMut};

impl Index<&FrameIndex> for CopyCompleteSemaphore {
    type Output = (VulkanSemaphore, u64);

    fn index(&self, index: &FrameIndex) -> &Self::Output {
        &self.semaphores[**index]
    }
}

impl IndexMut<&FrameIndex> for CopyCompleteSemaphore {
    fn index_mut(&mut self, index: &FrameIndex) -> &mut Self::Output {
        &mut self.semaphores[**index]
    }
}
