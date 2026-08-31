use crate::{
    Error, Result,
    render::{FixedRenderObjects, LitMaterialPushConstants, frame_graph::LitForwardRenderNode},
};
use alexandria::gpu::{
    VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanDevice, VulkanFormat,
    VulkanPushConstantRange, VulkanShaderStageFlag,
};

impl LitForwardRenderNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render) fn create_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create the descriptor set layout for the lighting data
        fixed_render_objects.add_descriptor_set_layout(
            &[
                VulkanDescriptorSetLayoutBinding::new(
                    0,
                    VulkanDescriptorType::UniformBuffer,
                    1,
                    VulkanShaderStageFlag::Fragment,
                ),
                VulkanDescriptorSetLayoutBinding::new(
                    1,
                    VulkanDescriptorType::StorageBuffer,
                    1,
                    VulkanShaderStageFlag::Fragment,
                ),
                VulkanDescriptorSetLayoutBinding::new(
                    2,
                    VulkanDescriptorType::StorageBuffer,
                    1,
                    VulkanShaderStageFlag::Fragment,
                ),
                VulkanDescriptorSetLayoutBinding::new(
                    3,
                    VulkanDescriptorType::StorageBuffer,
                    1,
                    VulkanShaderStageFlag::Fragment,
                ),
            ],
            1,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET_LAYOUT,
            device,
        )?;

        // Create the pipeline layout for lit materials
        fixed_render_objects.add_pipeline_layout(
            device
                .create_pipeline_layout(
                    0,
                    &[
                        fixed_render_objects.descriptor_set_layout(
                            FixedRenderObjects::CAMERA_DESCRIPTOR_SET_LAYOUT,
                        ),
                        fixed_render_objects.descriptor_set_layout(
                            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET_LAYOUT,
                        ),
                        fixed_render_objects.descriptor_set_layout(
                            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET_LAYOUT,
                        ),
                    ],
                    &[VulkanPushConstantRange::new(
                        VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                        0,
                        std::mem::size_of::<LitMaterialPushConstants>() as _,
                    )],
                )
                .map_err(Error::new_inner)?,
            FixedRenderObjects::LIT_OPAQUE_PIPELINE_LAYOUT,
        );

        Ok(())
    }
}
