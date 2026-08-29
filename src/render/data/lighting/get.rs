use crate::render::LightingData;
use alexandria::gpu::VulkanDescriptorSet;

impl LightingData {
    /// Get the descriptor set for the current frame
    pub fn descriptor_set(&self) -> &VulkanDescriptorSet {
        &self.descriptor_set
    }
}
