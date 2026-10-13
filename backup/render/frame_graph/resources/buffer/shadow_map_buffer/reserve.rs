use crate::{Result, render::ShadowMapBuffer};
use alexandria::{
    gpu::{VulkanAdapterMemoryProperties, VulkanDescriptorSet, VulkanDevice, VulkanSampler},
    math::Vector2u,
};

impl ShadowMapBuffer {
    /// Make sure the buffer has enough space for `capacity` shadow maps
    pub fn reserve(
        &mut self,
        capacity: usize,
        size: Vector2u,
        descriptor_sets: &[VulkanDescriptorSet],
        sampler: &VulkanSampler,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        if self.layer_image_views.len() < capacity || self.size != size {
            *self = ShadowMapBuffer::new_inner(
                self.image_name.clone(),
                self.memory_name.clone(),
                self.complete_image_view_name.clone(),
                self.layer_base_name.clone(),
                size,
                capacity,
                self.cascades,
                self.cube,
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
