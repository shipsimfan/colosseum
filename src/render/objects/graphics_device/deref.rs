use crate::render::GraphicsDevice;
use alexandria::gpu::VulkanDevice;
use std::ops::Deref;

impl Deref for GraphicsDevice {
    type Target = VulkanDevice;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl AsRef<VulkanDevice> for GraphicsDevice {
    fn as_ref(&self) -> &VulkanDevice {
        &self.device
    }
}
