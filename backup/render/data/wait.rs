use crate::{Error, Result, render::RenderData};
use alexandria::gpu::VulkanResult;

impl RenderData {
    /// Wait for all copy operations occuring on the buffer to complete
    pub fn wait_for_copy(&mut self) -> Result<()> {
        if self.copy_commands_sent {
            assert_eq!(
                self.copy_fence.wait(u64::MAX).map_err(Error::new_inner)?,
                VulkanResult::VkSuccess
            );
            self.copy_fence.reset().map_err(Error::new_inner)?;
            self.copy_commands_sent = false;
        }

        Ok(())
    }
}
