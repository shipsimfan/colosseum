#[cfg(debug_assertions)]
use std::ffi::CString;

use crate::{Error, Result, render::FixedRenderObjects};
use alexandria::gpu::{VulkanDescriptorPool, VulkanDevice};

impl FixedRenderObjects {
    /// Create a descriptor pool that can be used for a single frame
    pub(in crate::render) fn create_descriptor_pool(
        &self,
        #[cfg_attr(not(debug_assertions), allow(unused_variables))] index: usize,
        device: &VulkanDevice,
    ) -> Result<VulkanDescriptorPool> {
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut descriptor_pool = device
            .create_descriptor_pool(0, self.max_descriptor_sets, &self.descriptor_pool_sizes)
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut descriptor_pool,
                &CString::new(format!("Descriptor Pool {}", index)).unwrap(),
            )
            .map_err(Error::new_inner)?;
        Ok(descriptor_pool)
    }
}
