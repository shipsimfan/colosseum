use crate::render::DescriptorSetLayouts;

impl DescriptorSetLayouts {
    /// Create a new set of [`DescriptorSetLayouts`]
    pub fn new() -> DescriptorSetLayouts {
        DescriptorSetLayouts {
            layouts: Vec::new(),
        }
    }
}
