use crate::render::{
    DescriptorSetBinding, DescriptorSetLayoutInfo, GpuRenderable, GpuSpotLightMatrix, SpotLightDsl,
};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for SpotLightDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[
        DescriptorSetBinding::new_for::<GpuSpotLightMatrix, _>(
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
