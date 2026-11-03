use crate::render::{UnlitMaterial, public_data::materials::UnlitMaterialPushConstants};
use alexandria::gpu::{VulkanCommandBuffer, VulkanPipelineLayout, VulkanShaderStageFlag};
use colosseum_core::as_bytes;

impl UnlitMaterial {
    /// Bind the unlit material for rendering
    pub fn bind(
        &self,
        cmd_buffer: &mut VulkanCommandBuffer,
        pipeline_layout: &VulkanPipelineLayout,
        object_index: usize,
    ) {
        self.shader.bind(cmd_buffer);

        let push_constants = UnlitMaterialPushConstants {
            color: self.color,
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
