use crate::render::RenderMaterial;
use alexandria::gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint};

impl RenderMaterial {
    /// Bind the material to the render pass
    pub(in crate::render) fn bind(&self, cmd_buffer: &mut VulkanCommandBuffer) {
        cmd_buffer.cmd_bind_pipeline(VulkanPipelineBindPoint::Graphics, &self.pipeline);
    }
}
