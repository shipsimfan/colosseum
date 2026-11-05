use crate::render::LitForwardShader;
use alexandria::gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint};

impl LitForwardShader {
    /// Binds the shader for rendering
    pub(in crate::render::public_data::materials::lit_forward) fn bind(
        &self,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        cmd_buffer.cmd_bind_pipeline(VulkanPipelineBindPoint::Graphics, &self.pipeline);
    }
}
