use crate::render::{DescriptorSetBinding, DescriptorSetLayoutInfo, PostProcess};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for PostProcess {
    const BINDINGS: &[DescriptorSetBinding] = &[DescriptorSetBinding::new(
        VulkanDescriptorType::CombinedImageSampler,
        1,
        VulkanShaderStageFlag::Fragment,
    )];
}
