use crate::{
    Error, Result,
    render::{FixedRenderObjects, Pipeline},
};
use alexandria::gpu::{
    VulkanDescriptorPoolSize, VulkanDescriptorSetLayoutBinding, VulkanDevice, VulkanPipelineLayout,
    VulkanSampler,
};

impl FixedRenderObjects {
    /// Add a new pipeline layout
    pub(in crate::render) fn add_pipeline_layout(
        &mut self,
        pipeline_layout: VulkanPipelineLayout,
        index: usize,
    ) {
        assert_eq!(index, self.pipeline_layouts.len());
        self.pipeline_layouts.push(pipeline_layout);
    }

    /// Add a new pipeline
    pub(in crate::render) fn add_pipeline(&mut self, pipeline: Pipeline, index: usize) {
        assert_eq!(index, self.pipelines.len());
        self.pipelines.push(pipeline);
    }

    /// Add a new sampler
    pub(in crate::render) fn add_sampler(&mut self, sampler: VulkanSampler, index: usize) {
        assert_eq!(index, self.samplers.len());
        self.samplers.push(sampler);
    }

    /// Add a new descriptor set layout
    pub(in crate::render) fn add_descriptor_set_layout(
        &mut self,
        bindings: &[VulkanDescriptorSetLayoutBinding],
        quantity: u32,
        index: usize,
        device: &VulkanDevice,
    ) -> Result<()> {
        assert_eq!(index, self.descriptor_set_layouts.len());

        let descriptor_set_layout = device
            .create_descriptor_set_layout(0, bindings)
            .map_err(Error::new_inner)?;

        self.descriptor_set_layouts.push(descriptor_set_layout);

        self.max_descriptor_sets += quantity;
        'binding_loop: for binding in bindings {
            let count = binding.descriptor_count() * quantity;
            for pool_size in &mut self.descriptor_pool_sizes {
                if pool_size.r#type() == binding.descriptor_type() {
                    *pool_size.count_mut() += count;
                    continue 'binding_loop;
                }
            }

            self.descriptor_pool_sizes
                .push(VulkanDescriptorPoolSize::new(
                    binding.descriptor_type(),
                    count,
                ));
        }

        Ok(())
    }
}
