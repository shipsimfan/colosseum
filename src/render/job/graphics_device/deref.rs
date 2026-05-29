use crate::render::job::GraphicsDevice;
use alexandria::gpu::VulkanDevice;
use std::ops::Deref;

impl<'surface> Deref for GraphicsDevice<'surface> {
    type Target = VulkanDevice;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
