use alexandria::gpu::{VulkanDescriptorType, VulkanShaderStageFlags};

mod get;
mod new;
mod to_vulkan;

/// A single binding within a descriptor set
pub struct DescriptorSetBinding {
    /// The type of descriptor this binding represents
    r#type: VulkanDescriptorType,

    /// The number of descriptors in this binding
    count: u32,

    /// The shader stages that can access this binding
    stage_flags: VulkanShaderStageFlags,
}
