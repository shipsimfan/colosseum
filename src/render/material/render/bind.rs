use crate::render::{
    LitMaterialPushConstants, MaterialKind, ObjectData, RenderMaterial, UnlitMaterialPushConstants,
    as_bytes,
};
use alexandria::gpu::{
    GpuAddress, VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanPipelineLayout,
    VulkanShaderStageFlag,
};

impl RenderMaterial {
    /// Bind the material to the render pass
    pub(in crate::render) fn bind(
        &self,
        cmd_buffer: &mut VulkanCommandBuffer,
        pipeline_layout: &VulkanPipelineLayout,
        object_data: GpuAddress<ObjectData>,
    ) {
        cmd_buffer.cmd_bind_pipeline(VulkanPipelineBindPoint::Graphics, &self.pipeline);

        match self.kind {
            MaterialKind::UnlitOpaque => {
                let push_constants = UnlitMaterialPushConstants {
                    color: self.color,
                    object_data,
                };
                cmd_buffer.cmd_push_constants(
                    pipeline_layout,
                    VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                    0,
                    unsafe { as_bytes(&push_constants) },
                );
            }
            MaterialKind::LitOpaque => {
                let push_constants = LitMaterialPushConstants {
                    color: self.color,
                    specular_strength: self.specular_strength,
                    shininess: self.shininess,
                    object_data,
                };
                cmd_buffer.cmd_push_constants(
                    pipeline_layout,
                    VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                    0,
                    unsafe { as_bytes(&push_constants) },
                );
            }
        }
    }
}
