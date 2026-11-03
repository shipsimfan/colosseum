use crate::render::{DescriptorSetBinding, DescriptorSetLayout};
use alexandria::gpu::VulkanDescriptorSetLayout;
use std::any::TypeId;

impl DescriptorSetLayout {
    /// Get the name of the descriptor set layout
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the type ID of the descriptor set layout
    pub fn r#type(&self) -> TypeId {
        self.r#type
    }

    /// Get the handle of the descriptor set layout
    pub fn handle(&self) -> &VulkanDescriptorSetLayout {
        &self.handle
    }

    /// Get the bindings of the descriptor set layout
    pub fn bindings(&self) -> &'static [DescriptorSetBinding] {
        &self.bindings
    }

    /// Get the data bindings of the descriptor set layout
    pub fn data_bindings(&self) -> &[(TypeId, &'static str, u32)] {
        &self.data_bindings
    }
}
