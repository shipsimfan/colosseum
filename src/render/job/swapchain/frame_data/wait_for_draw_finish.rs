use crate::{Error, Result, render::job::swapchain::FrameData};
use alexandria::gpu::VulkanResult;

impl FrameData {
    /// Wait for the GPU to finish executing the command buffer for this frame, and reset the fence
    /// for the next frame
    pub fn wait_for_draw_finish(&mut self) -> Result<()> {
        assert_eq!(
            self.draw_fence.wait(u64::MAX).map_err(Error::new_inner)?,
            VulkanResult::VkSuccess
        );
        self.draw_fence.reset().map_err(Error::new_inner)
    }
}
