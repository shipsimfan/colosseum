use crate::render::DescriptorSetLayouts;
use alexandria::gpu::VulkanDescriptorSetLayout;
use std::any::TypeId;

impl DescriptorSetLayouts {
    /// Get a descriptor set layout by its type
    pub fn get<T: 'static>(&self) -> &VulkanDescriptorSetLayout {
        let type_id = TypeId::of::<T>();
        for (id, layout) in &self.layouts {
            if *id == type_id {
                return layout;
            }
        }

        panic!(
            "Descriptor set layout not found for {}",
            std::any::type_name::<T>()
        );
    }

    /// Get a descriptor set layout by its type ID
    pub fn get_by_id(&self, id: TypeId) -> &VulkanDescriptorSetLayout {
        for (layout_id, layout) in &self.layouts {
            if *layout_id == id {
                return layout;
            }
        }

        panic!("Descriptor set layout not found for {:?}", id);
    }
}
