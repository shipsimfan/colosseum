use crate::{
    Error, Result,
    render::{FixedRenderObjects, UnlitMaterialPushConstants, frame_graph::UnlitForwardRenderNode},
};
use alexandria::gpu::{
    VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanDevice, VulkanFormat,
    VulkanPushConstantRange, VulkanShaderStageFlag,
};

impl UnlitForwardRenderNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create the descriptor set layout for the renderables
        fixed_render_objects.add_descriptor_set_layout(
            &[VulkanDescriptorSetLayoutBinding::new(
                0,
                VulkanDescriptorType::StorageBuffer,
                1,
                VulkanShaderStageFlag::Vertex,
            )],
            1,
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET_LAYOUT,
            device,
        )?;

        // Create the pipeline layout for this node
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
                    ],
                    &[VulkanPushConstantRange::new(
                        VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                        0,
                        std::mem::size_of::<UnlitMaterialPushConstants>() as _,
                    )],
                )
                .map_err(Error::new_inner)?,
            FixedRenderObjects::UNLIT_OPAQUE_PIPELINE_LAYOUT,
        );

        Ok(())
    }
}
