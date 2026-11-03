use crate::render::UnlitShader;
use alexandria::gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint};

impl UnlitShader {
    /// Bind this shader for rendering
    pub(in crate::render::public_data::materials::unlit) fn bind(
        &self,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        cmd_buffer.cmd_bind_pipeline(VulkanPipelineBindPoint::Graphics, &self.pipeline);
    }
}
