use crate::render::{DescriptorSetLayout, DescriptorSetLayoutInfo};
use alexandria::gpu::VulkanDescriptorSetLayout;
use std::any::TypeId;

impl DescriptorSetLayout {
    /// Create a new [`DescriptorSetLayout`]
    pub fn new<T: DescriptorSetLayoutInfo>(
        name: &'static str,
        handle: VulkanDescriptorSetLayout,
    ) -> DescriptorSetLayout {
        DescriptorSetLayout {
            name,
            r#type: TypeId::of::<T>(),
            handle,
            bindings: T::BINDINGS,
            data_bindings: T::BINDINGS
                .iter()
                .enumerate()
                .filter_map(|(i, binding)| {
                    binding
                        .data_buffer()
                        .map(|(r#type, name)| (r#type, name, i as u32))
                })
                .collect(),
        }
    }
}
