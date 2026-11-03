use crate::render::DescriptorSetBinding;
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlags};
use std::any::TypeId;

impl DescriptorSetBinding {
    /// Create a new [`DescriptorSetBinding`]
    pub const fn new<F: const Into<VulkanShaderStageFlags>>(
        r#type: VulkanDescriptorType,
        count: u32,
        stage_flags: F,
    ) -> DescriptorSetBinding {
        DescriptorSetBinding {
            r#type,
            count,
            stage_flags: stage_flags.into(),
            data_buffer: None,
        }
    }

    /// Create a new [`DescriptorSetBinding`]
    pub const fn new_for<T: 'static, F: const Into<VulkanShaderStageFlags>>(
        r#type: VulkanDescriptorType,
        count: u32,
        stage_flags: F,
    ) -> DescriptorSetBinding {
        DescriptorSetBinding {
            r#type,
            count,
            stage_flags: stage_flags.into(),
            data_buffer: Some((TypeId::of::<T>(), std::any::type_name::<T>())),
        }
    }
}
