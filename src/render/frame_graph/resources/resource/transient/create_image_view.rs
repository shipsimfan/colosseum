use crate::{
    Result,
    render::{GraphicsDevice, frame_graph::FrameGraphTransientResourceInfo},
};
use alexandria::gpu::{
    VulkanComponentMapping, VulkanDeviceMemory, VulkanImage, VulkanImageView, VulkanImageViewType,
    VulkanMemoryRequirements,
};
use colosseum_core::new_error;

impl FrameGraphTransientResourceInfo {
    /// Create a new image view for this resource
    pub fn create_image_view(
        &self,
        mut image: VulkanImage,
        memory_requirements: &VulkanMemoryRequirements,
        memory: &VulkanDeviceMemory,
        mut offset: u64,
        frame_index: usize,
        device: &GraphicsDevice,
    ) -> Result<(VulkanImageView, u64)> {
        // Bind memory to the image
        offset = offset.next_multiple_of(memory_requirements.alignment());
        image.bind_memory(memory, offset).map_err(|error| {
            crate::new_error!("unable to bind memory for \"{}\" - {}", self.name, error)
        })?;

        // Create the image view
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut image_view = image
            .create_image_view(
                0,
                VulkanImageViewType::_2d,
                self.format,
                VulkanComponentMapping::default(),
                self.aspect_mask(),
                0,
                1,
                0,
                1,
            )
            .map_err(|error| {
                new_error!(
                    "unable to create an image view for \"{}\" - {}",
                    self.name,
                    error
                )
            })?;

        // Name the image view
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut image_view, &self.image_view_name[frame_index])
            .map_err(|error| {
                new_error!(
                    "unable to set image view name for \"{}\" - {}",
                    self.name,
                    error
                )
            })?;

        Ok((image_view, offset + *memory_requirements.size()))
    }
}
