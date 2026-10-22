use crate::render::DescriptorSetBinding;
use alexandria::gpu::VulkanDescriptorType;

impl DescriptorSetBinding {
    /// Get the descriptor type of this binding
    pub fn r#type(&self) -> VulkanDescriptorType {
        self.r#type
    }

    /// Get the descriptor count of this binding
    pub fn count(&self) -> u32 {
        self.count
    }
}
