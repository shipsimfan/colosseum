use crate::render::{
    DescriptorSetBinding, DescriptorSetLayoutInfo, GpuPointLight, GpuPointLightMatrix,
    GpuRenderable, PointLightDsl,
};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for PointLightDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[
        DescriptorSetBinding::new_for::<GpuPointLightMatrix, _>(
            VulkanDescriptorType::UniformBuffer,
            1,
            VulkanShaderStageFlag::Vertex,
        ),
        DescriptorSetBinding::new_for::<GpuRenderable, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Vertex,
        ),
        DescriptorSetBinding::new_for::<GpuPointLight, _>(
            VulkanDescriptorType::UniformBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
    ];
}
