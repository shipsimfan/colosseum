use crate::{
    Error, Result,
    render::{
        FrameGraphNode, FrameGraphTransientBuffer,
        job::{GraphicsDevice, RenderToken},
    },
};
use alexandria::gpu::{
    VulkanCommandBuffer, VulkanCommandBufferSubmitInfo, VulkanPipelineStageFlag, VulkanSemaphore,
    VulkanSemaphoreSubmitInfo, VulkanSubmitInfo,
};

impl GraphicsDevice {
    /// Copy the required data from staging buffers to device local buffers
    pub fn copy_data(
        &mut self,
        token: &RenderToken,
        transient_buffer: &mut FrameGraphTransientBuffer,
        copy_complete_semaphore: &VulkanSemaphore,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) -> Result<()> {
        cmd_buffer.begin().map_err(Error::new_inner)?;

        FrameGraphNode::copy_data(
            &mut self.render_data[token.frame_index()],
            &mut transient_buffer.device_buffers,
            cmd_buffer,
            &self.device,
            &self.memory_properties,
        )?;

        cmd_buffer.end().map_err(Error::new_inner)?;

        self.queue
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[],
                    &[VulkanCommandBufferSubmitInfo::new(cmd_buffer, 0)],
                    &[VulkanSemaphoreSubmitInfo::new(
                        copy_complete_semaphore,
                        0,
                        VulkanPipelineStageFlag::Copy,
                        0,
                    )],
                )],
                Some(self.render_data[token.frame_index()].copy_fence()),
            )
            .map_err(Error::new_inner)
    }
}
