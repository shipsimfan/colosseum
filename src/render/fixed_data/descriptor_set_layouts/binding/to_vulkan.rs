use crate::render::DescriptorSetBinding;
use alexandria::gpu::VulkanDescriptorSetLayoutBinding;

impl DescriptorSetBinding {
    /// Convert this binding to its Vulkan representation
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn to_vulkan(
        &self,
        index: u32,
    ) -> VulkanDescriptorSetLayoutBinding {
        VulkanDescriptorSetLayoutBinding::new(index, self.r#type, self.count, self.stage_flags)
    }
}
