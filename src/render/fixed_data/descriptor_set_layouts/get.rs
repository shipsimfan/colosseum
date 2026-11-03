use crate::render::DescriptorSetLayouts;
use alexandria::gpu::VulkanDescriptorSetLayout;
use std::any::TypeId;

impl DescriptorSetLayouts {
    /// Get a descriptor set layout by its type
    pub fn get<T: 'static>(&self) -> &VulkanDescriptorSetLayout {
        let type_id = TypeId::of::<T>();
        for descriptor_set_layout in &self.layouts {
            if descriptor_set_layout.r#type() == type_id {
                return &descriptor_set_layout.handle();
            }
        }

        panic!(
            "Descriptor set layout not found for {}",
            std::any::type_name::<T>()
        );
    }

    /// Get a descriptor set layout by its type ID
    pub fn get_by_id(&self, id: TypeId) -> &VulkanDescriptorSetLayout {
        for descriptor_set_layout in &self.layouts {
            if descriptor_set_layout.r#type() == id {
                return &descriptor_set_layout.handle();
            }
        }

        panic!("Descriptor set layout not found for {:?}", id);
    }
}
