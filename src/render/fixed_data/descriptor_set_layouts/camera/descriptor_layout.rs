use crate::render::{CameraDsl, DescriptorSetBinding, DescriptorSetLayoutInfo, GpuCamera};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for CameraDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[DescriptorSetBinding::new_for::<GpuCamera, _>(
        VulkanDescriptorType::UniformBuffer,
        1,
        VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
    )];
}
