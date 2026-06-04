use crate::{Error, Result, render::job::GraphicsDevice};
use alexandria::gpu::VulkanCommandBufferLevel;

impl GraphicsDevice {
    /// Make sure at least `num` command buffers are allocated in the command pool
    pub fn reserve_command_buffers(&mut self, num: usize) -> Result<()> {
        if self.command_buffers.len() < num {
            for _ in self.command_buffers.len()..num {
                self.command_buffers.push(
                    self.command_pool
                        .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
                        .map_err(Error::new_inner)?,
                );
            }
        }

        Ok(())
    }
}
