use crate::render::{LitForwardMaterial, public_data::materials::LitForwardMaterialPushConstants};
use alexandria::gpu::{VulkanCommandBuffer, VulkanPipelineLayout, VulkanShaderStageFlag};
use colosseum_core::as_bytes;

impl LitForwardMaterial {
    /// Bind the lit forward material for rendering
    pub(in crate::render) fn bind(
        &self,
        cmd_buffer: &mut VulkanCommandBuffer,
        pipeline_layout: &VulkanPipelineLayout,
        object_index: usize,
    ) {
        self.shader.bind(cmd_buffer);

        let push_constants = LitForwardMaterialPushConstants {
            color: self.color.with_alpha(1.0),
            specular_strength: self.specular_strength,
            shininess: self.shininess,
            object_index: object_index as _,
        };
        cmd_buffer.cmd_push_constants(
            pipeline_layout,
            VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
            0,
            as_bytes(&push_constants),
        );
    }
}
