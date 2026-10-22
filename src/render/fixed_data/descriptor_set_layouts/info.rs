use crate::render::DescriptorSetBinding;

/// The description of a descriptor set layout
pub(in crate::render) trait DescriptorSetLayoutInfo: 'static {
    /// The bindings that describe the layout of the descriptor set
    const BINDINGS: &[DescriptorSetBinding];
}
