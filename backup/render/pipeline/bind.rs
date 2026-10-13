use crate::render::Pipeline;
use alexandria::gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint};

impl Pipeline {
    /// Bind the material to the render pass
    pub(in crate::render) fn bind(&self, cmd_buffer: &mut VulkanCommandBuffer) {
        cmd_buffer.cmd_bind_pipeline(VulkanPipelineBindPoint::Graphics, &self.pipeline);
    }
}
