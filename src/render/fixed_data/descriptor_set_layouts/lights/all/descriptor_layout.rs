use crate::render::{DescriptorSetBinding, DescriptorSetLayoutInfo, LightsDsl};
use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlag};

impl DescriptorSetLayoutInfo for LightsDsl {
    const BINDINGS: &[DescriptorSetBinding] = &[];
}
