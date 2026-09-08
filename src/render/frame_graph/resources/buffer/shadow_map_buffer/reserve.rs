use crate::{Result, render::ShadowMapBuffer};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanDescriptorSet, VulkanDevice, VulkanSampler,
};

impl ShadowMapBuffer {
    /// Make sure the buffer has enough space for `capacity` shadow maps
    pub fn reserve(
        &mut self,
        capacity: usize,
        descriptor_sets: &[VulkanDescriptorSet],
        sampler: &VulkanSampler,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        if self.layer_image_views.len() < capacity {
            #[allow(invalid_value)]
            let mut name = unsafe { std::mem::zeroed() };
            std::mem::swap(&mut name, &mut self.name);

            *self = ShadowMapBuffer::new_inner(
                name,
                self.size,
                capacity,
                self.descriptor_set,
                self.binding,
                descriptor_sets,
                sampler,
                device,
                memory_properties,
            )?;
        }

        Ok(())
    }
}
