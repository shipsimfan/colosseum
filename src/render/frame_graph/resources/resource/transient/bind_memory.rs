use crate::{
    Error, Result,
    render::frame_graph::{FrameGraphDynamicTransientResourceInfo, FrameGraphTransientResource},
};
use alexandria::gpu::{VulkanComponentMapping, VulkanDeviceMemory, VulkanImageViewType};

impl FrameGraphTransientResource {
    /// Bind memory to a resource
    pub fn bind_memory(
        &mut self,
        info: &FrameGraphDynamicTransientResourceInfo,
        memory: &VulkanDeviceMemory,
        mut offset: u64,
    ) -> Result<u64> {
        // Bind the memory to the image
        offset = offset.next_multiple_of(self.memory_requirements.alignment());
        self.image
            .bind_memory(memory, offset)
            .map_err(Error::new_inner)?;

        // Create the image view
        self.image_view = Some(
            self.image
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
                .map_err(Error::new_inner)?,
        );

        Ok(offset + *self.memory_requirements.size())
    }
}
