use crate::render::{
    DescriptorSetBinding, DescriptorSetLayoutInfo, DirectionalLightDsl, GpuDirectionalLightMatrix,
    GpuRenderable,
};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for DirectionalLightDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[
        DescriptorSetBinding::new_for::<GpuDirectionalLightMatrix, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Vertex,
        ),
        DescriptorSetBinding::new_for::<GpuRenderable, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Vertex,
        ),
    ];
}
