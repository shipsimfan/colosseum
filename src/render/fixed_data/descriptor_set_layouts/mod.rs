use alexandria::gpu::VulkanDescriptorSetLayout;
use std::any::TypeId;

mod binding;
mod info;

mod camera;
mod post_process;

mod create;
mod get;
mod new;

pub(in crate::render) use binding::*;
pub(in crate::render) use info::*;

pub(in crate::render) use camera::*;
pub(in crate::render) use post_process::*;

/// A wrapper around a Vulkan descriptor set layout
pub(in crate::render) struct DescriptorSetLayouts {
    /// The descriptor set layouts that have been created
    layouts: Vec<(TypeId, VulkanDescriptorSetLayout)>,
}
