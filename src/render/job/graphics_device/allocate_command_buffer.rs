use crate::{Error, Result, render::job::GraphicsDevice};
use alexandria::gpu::VulkanCommandBuffer;

impl<'surface> GraphicsDevice<'surface> {
    pub fn allocate_command_buffer(&self) -> Result<VulkanCommandBuffer> {
        self.command_pool
            .allocate_command_buffer()
            .map_err(Error::new_inner)
    }
}
