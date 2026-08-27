use crate::render::{
    FixedRenderObjects, RenderData, RenderObjects,
    frame_graph::{FrameGraphResources, GammaCorrectionNode},
};
use alexandria::gpu::{
    VulkanDescriptorImageInfo, VulkanDescriptorType, VulkanDevice, VulkanImageLayout,
    VulkanWriteDescriptorSet,
};

impl GammaCorrectionNode {
    /// Update the descriptor sets for this node
    pub(in crate::render::frame_graph) fn update_descriptor_sets(
        &self,
        render_data: &RenderData,
        render_objects: &RenderObjects,
        resources: &FrameGraphResources,
        device: &VulkanDevice,
    ) {
        device.update_descriptor_sets(
            &[VulkanWriteDescriptorSet::new(
                render_data
                    .post_process_descriptor_set(RenderData::GAMMA_CORRECTION_DESCRIPTOR_SET),
                0,
                0,
                VulkanDescriptorType::CombinedImageSampler,
                &[VulkanDescriptorImageInfo::new(
                    render_objects
                        .fixed()
                        .sampler(FixedRenderObjects::LINEAR_CLAMP_SAMPLER),
                    resources.get(self.input).image_view(),
                    VulkanImageLayout::ShaderReadOnlyOptimal,
                )],
                &[],
            )],
            &[],
        );
    }
}
