use crate::render::{
    DescriptorSetBinding, DescriptorSetLayoutInfo, GpuDirectionalLight, GpuDirectionalLightMatrix,
    GpuPointLight, GpuPointLightMatrix, GpuSpotLight, GpuSpotLightMatrix, LightMetadata, LightsDsl,
};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for LightsDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[
        DescriptorSetBinding::new_for::<LightMetadata, _>(
            VulkanDescriptorType::UniformBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        // Directional Lights
        DescriptorSetBinding::new_for::<GpuDirectionalLight, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        DescriptorSetBinding::new_for::<GpuDirectionalLightMatrix, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        DescriptorSetBinding::new(
            VulkanDescriptorType::CombinedImageSampler,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        // Point Lights
        DescriptorSetBinding::new_for::<GpuPointLight, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        DescriptorSetBinding::new_for::<GpuPointLightMatrix, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        DescriptorSetBinding::new(
            VulkanDescriptorType::CombinedImageSampler,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        // Spot Lights
        DescriptorSetBinding::new_for::<GpuSpotLight, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        DescriptorSetBinding::new_for::<GpuSpotLightMatrix, _>(
            VulkanDescriptorType::StorageBuffer,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
        DescriptorSetBinding::new(
            VulkanDescriptorType::CombinedImageSampler,
            1,
            VulkanShaderStageFlag::Fragment,
        ),
    ];
}
