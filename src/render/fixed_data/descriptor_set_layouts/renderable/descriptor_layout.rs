use crate::render::{DescriptorSetBinding, DescriptorSetLayoutInfo, GpuRenderable, RenderableDsl};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for RenderableDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[DescriptorSetBinding::new_for::<GpuRenderable, _>(
        VulkanDescriptorType::StorageBuffer,
        1,
        VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
    )];
}
