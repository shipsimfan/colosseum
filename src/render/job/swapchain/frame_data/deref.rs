use crate::render::job::swapchain::FrameData;
use alexandria::gpu::VulkanCommandBuffer;
use std::ops::{Deref, DerefMut};

impl Deref for FrameData {
    type Target = VulkanCommandBuffer;

    fn deref(&self) -> &Self::Target {
        &self.command_buffer
    }
}

impl DerefMut for FrameData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.command_buffer
    }
}
