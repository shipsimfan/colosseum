use crate::{Error, Result, render::FixedRenderObjects};
use alexandria::gpu::{VulkanDescriptorPool, VulkanDevice};

impl FixedRenderObjects {
    /// Create a descriptor pool that can be used for a single frame
    pub(in crate::render) fn create_descriptor_pool(
        &self,
        device: &VulkanDevice,
    ) -> Result<VulkanDescriptorPool> {
        device
            .create_descriptor_pool(0, self.max_descriptor_sets, &self.descriptor_pool_sizes)
            .map_err(Error::new_inner)
    }
}
