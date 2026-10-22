use crate::render::{
    DescriptorSetBinding, DescriptorSetLayoutInfo, frame_graph::FrameGraphBuilder,
};
use alexandria::gpu::VulkanDescriptorPoolSize;
use std::{any::TypeId, borrow::Cow};

impl<'a> FrameGraphBuilder<'a> {
    /// Create a new descriptor set for the frame graph
    pub fn create_descriptor_set<Layout: DescriptorSetLayoutInfo, S: Into<Cow<'static, str>>>(
        &mut self,
        name: S,
    ) -> usize {
        for binding in Layout::BINDINGS {
            self.add_descriptors(binding);
        }

        self.descriptor_sets
            .push((TypeId::of::<Layout>(), name.into(), Layout::BINDINGS));
        self.descriptor_sets.len() - 1
    }

    /// Add the required descriptors for a given binding to the descriptor pool
    fn add_descriptors(&mut self, binding: &DescriptorSetBinding) {
        for pool_size in &mut self.descriptors {
            if pool_size.r#type() == binding.r#type() {
                *pool_size.count_mut() += binding.count();
                return;
            }
        }

        self.descriptors.push(VulkanDescriptorPoolSize::new(
            binding.r#type(),
            binding.count(),
        ));
    }
}
