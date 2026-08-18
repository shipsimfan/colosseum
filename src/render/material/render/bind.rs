use crate::render::{ObjectData, RenderMaterial, as_bytes};
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
        let mut bytes = [0u8; RenderMaterial::PUSH_CONSTANT_SIZE];
        bytes[..RenderMaterial::DATA_SIZE].copy_from_slice(unsafe { as_bytes(&self.color) });
        bytes[RenderMaterial::DATA_SIZE
            ..RenderMaterial::DATA_SIZE + std::mem::size_of::<GpuAddress<ObjectData>>()]
            .copy_from_slice(unsafe { as_bytes(&object_data) });

        cmd_buffer.cmd_bind_pipeline(VulkanPipelineBindPoint::Graphics, &self.pipeline);

        cmd_buffer.cmd_push_constants(
            pipeline_layout,
            VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
            0,
            &bytes,
        );
    }
}
