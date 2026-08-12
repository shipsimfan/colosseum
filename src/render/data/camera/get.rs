use crate::render::CameraRenderData;
use alexandria::gpu::VulkanDescriptorSet;

impl CameraRenderData {
    /// Get a reference to the descriptor set containing the camera shader data
    pub(in crate::render) fn descriptor_set(&self) -> &VulkanDescriptorSet {
        &self.descriptor_set
    }
}
