use crate::render::RenderQueue;
use alexandria::gpu::VulkanQueue;
use std::ops::{Deref, DerefMut};

impl Deref for RenderQueue {
    type Target = VulkanQueue;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl AsRef<VulkanQueue> for RenderQueue {
    fn as_ref(&self) -> &VulkanQueue {
        &self.queue
    }
}

impl DerefMut for RenderQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

impl AsMut<VulkanQueue> for RenderQueue {
    fn as_mut(&mut self) -> &mut VulkanQueue {
        &mut self.queue
    }
}
