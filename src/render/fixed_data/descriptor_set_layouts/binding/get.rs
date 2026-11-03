use crate::render::DescriptorSetBinding;
use alexandria::gpu::VulkanDescriptorType;
use std::any::TypeId;

impl DescriptorSetBinding {
    /// Get the descriptor type of this binding
    pub fn r#type(&self) -> VulkanDescriptorType {
        self.r#type
    }

    /// Get the descriptor count of this binding
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Is this binding for a data buffer
    pub fn data_buffer(&self) -> Option<(TypeId, &'static str)> {
        self.data_buffer
    }
}
