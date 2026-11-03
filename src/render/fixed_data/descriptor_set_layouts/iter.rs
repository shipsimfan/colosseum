use crate::render::{DescriptorSetLayout, DescriptorSetLayouts};

impl DescriptorSetLayouts {
    /// Returns an iterator over the descriptor set layouts
    pub fn iter(&self) -> std::slice::Iter<'_, DescriptorSetLayout> {
        self.layouts.iter()
    }
}

impl<'a> IntoIterator for &'a DescriptorSetLayouts {
    type Item = &'a DescriptorSetLayout;
    type IntoIter = std::slice::Iter<'a, DescriptorSetLayout>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
