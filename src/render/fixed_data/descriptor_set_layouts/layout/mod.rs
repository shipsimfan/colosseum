use crate::render::DescriptorSetBinding;
use alexandria::gpu::VulkanDescriptorSetLayout;
use std::any::TypeId;

mod get;
mod new;

/// A single descriptor set layout
pub(in crate::render) struct DescriptorSetLayout {
    /// The name of the descriptor set layout
    name: &'static str,

    /// The type ID of the descriptor set layout
    r#type: TypeId,

    /// The underlying Vulkan descriptor set layout
    handle: VulkanDescriptorSetLayout,

    /// The bindings for the data buffers in this layout
    bindings: &'static [DescriptorSetBinding],

    /// The set of data bindings this layout contains
    data_bindings: Vec<(TypeId, &'static str, u32)>,
}
