use crate::render::{
    FixedRenderObjects, RenderObjects,
    frame_graph::{FrameGraphResources, ToneMapNode},
};
use alexandria::gpu::{
    VulkanDescriptorImageInfo, VulkanDescriptorType, VulkanDevice, VulkanImageLayout,
    VulkanWriteDescriptorSet,
};

impl ToneMapNode {
    /// Update the descriptor sets for this node
    pub(in crate::render::frame_graph) fn update_descriptor_sets(
        &self,
        render_objects: &RenderObjects,
        resources: &FrameGraphResources,
        device: &VulkanDevice,
    ) {
        device.update_descriptor_sets(
            &[VulkanWriteDescriptorSet::new(
                resources.descriptor_set(FixedRenderObjects::TONE_MAP_DESCRIPTOR_SET),
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
