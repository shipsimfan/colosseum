use crate::render::{DescriptorSetBinding, DescriptorSetLayoutInfo, RenderableDsl};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for RenderableDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[DescriptorSetBinding::new(
        VulkanDescriptorType::StorageBuffer,
        1,
        VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
    )];
}
