use crate::render::{CameraDsl, DescriptorSetBinding, DescriptorSetLayoutInfo};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for CameraDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[DescriptorSetBinding::new(
        VulkanDescriptorType::UniformBuffer,
        1,
        VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
    )];
}
