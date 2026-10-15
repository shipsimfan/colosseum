use crate::render::{AcquireImageSemaphore, FrameIndex};
use alexandria::gpu::VulkanSemaphore;
use std::ops::{Index, IndexMut};

impl Index<&FrameIndex> for AcquireImageSemaphore {
    type Output = VulkanSemaphore;

    fn index(&self, index: &FrameIndex) -> &Self::Output {
        &self.semaphores[**index]
    }
}

impl IndexMut<&FrameIndex> for AcquireImageSemaphore {
    fn index_mut(&mut self, index: &FrameIndex) -> &mut Self::Output {
        &mut self.semaphores[**index]
    }
}
