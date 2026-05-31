use crate::render::FrameContext;
use alexandria::gpu::VulkanCommandBuffer;
use std::ops::{Deref, DerefMut};

impl<'frame, 'surface> Deref for FrameContext<'frame, 'surface> {
    type Target = VulkanCommandBuffer;

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<'frame, 'surface> DerefMut for FrameContext<'frame, 'surface> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}
