use crate::render::DescriptorSetBinding;
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlags};

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
        }
    }
}
