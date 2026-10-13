use crate::render::job::GraphicsDevice;
use alexandria::gpu::VulkanDevice;
use std::ops::Deref;

impl Deref for GraphicsDevice {
    type Target = VulkanDevice;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
