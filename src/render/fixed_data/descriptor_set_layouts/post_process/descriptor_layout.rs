use crate::render::{DescriptorSetBinding, DescriptorSetLayoutInfo, PostProcessDsl};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for PostProcessDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[DescriptorSetBinding::new(
        VulkanDescriptorType::CombinedImageSampler,
        1,
        VulkanShaderStageFlag::Fragment,
    )];
}
