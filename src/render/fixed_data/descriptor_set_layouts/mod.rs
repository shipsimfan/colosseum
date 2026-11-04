mod binding;
mod info;
mod layout;

mod camera;
mod lights;
mod post_process;
mod renderable;

mod create;
mod get;
mod iter;
mod new;

pub(in crate::render) use binding::*;
pub(in crate::render) use info::*;
pub(in crate::render) use layout::*;

pub(in crate::render) use camera::*;
pub(in crate::render) use lights::*;
pub(in crate::render) use post_process::*;
pub(in crate::render) use renderable::*;

/// A wrapper around a Vulkan descriptor set layout
pub(in crate::render) struct DescriptorSetLayouts {
    /// The descriptor set layouts that have been created
    layouts: Vec<DescriptorSetLayout>,
}
