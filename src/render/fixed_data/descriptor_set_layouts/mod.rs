use alexandria::gpu::VulkanDescriptorSetLayout;
use std::any::TypeId;

mod binding;
mod info;

mod create;
mod get;
mod new;

pub(in crate::render) use binding::*;
pub(in crate::render) use info::*;

/// A wrapper around a Vulkan descriptor set layout
pub(in crate::render) struct DescriptorSetLayouts {
    /// The descriptor set layouts that have been created
    layouts: Vec<(TypeId, VulkanDescriptorSetLayout)>,
}
