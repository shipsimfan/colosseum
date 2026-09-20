#[cfg(debug_assertions)]
use std::ffi::CString;

use crate::{
    Error, Result,
    render::frame_graph::{FrameGraphDynamicTransientResourceInfo, FrameGraphTransientResource},
};
use alexandria::gpu::{
    VulkanComponentMapping, VulkanDevice, VulkanDeviceMemory, VulkanImageViewType,
};

impl FrameGraphTransientResource {
    /// Bind memory to a resource
    pub fn bind_memory(
        &mut self,
        info: &FrameGraphDynamicTransientResourceInfo,
        memory: &VulkanDeviceMemory,
        mut offset: u64,
        device: &VulkanDevice,
    ) -> Result<u64> {
        // Bind the memory to the image
        offset = offset.next_multiple_of(self.memory_requirements.alignment());
        self.image
            .bind_memory(memory, offset)
            .map_err(Error::new_inner)?;

        // Create the image view
        let mut image_view = self
            .image
            .create_image_view(
                0,
                VulkanImageViewType::_2d,
                info.format(),
                VulkanComponentMapping::default(),
                self.aspect_mask,
                0,
                1,
                0,
                1,
            )
            .map_err(Error::new_inner)?;

        #[cfg(debug_assertions)]
        let name = CString::new(format!("{} Image View", info.name())).unwrap();
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut image_view, &name)
            .map_err(Error::new_inner)?;

        self.image_view = Some(image_view);

        Ok(offset + *self.memory_requirements.size())
    }
}
