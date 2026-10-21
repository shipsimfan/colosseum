use crate::{
    Result,
    render::{GraphicsDevice, frame_graph::FrameGraphTransientResourceInfo},
};
use alexandria::{
    gpu::{
        VulkanImage, VulkanImageLayout, VulkanImageTiling, VulkanImageType, VulkanSampleCountFlag,
        VulkanSharingMode,
    },
    math::Vector2u,
};
use colosseum_core::new_error;

impl FrameGraphTransientResourceInfo {
    /// Create a new image for this resource
    pub fn create_image(
        &self,
        size: Vector2u,
        frame_index: usize,
        device: &GraphicsDevice,
    ) -> Result<VulkanImage> {
        // Create the image
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut image = device
            .create_image(
                0,
                VulkanImageType::_2d,
                self.format,
                size.extend(1),
                1,
                1,
                VulkanSampleCountFlag::_1,
                VulkanImageTiling::Optimal,
                self.usage(),
                VulkanSharingMode::Exclusive,
                &[],
                VulkanImageLayout::Undefined,
            )
            .map_err(|error| {
                new_error!(
                    "unable to create an image for \"{}\" - {}",
                    self.name,
                    error
                )
            })?;

        // Name the image
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut image, &self.image_name[frame_index])
            .map_err(|error| {
                new_error!("unable to set image name for \"{}\" - {}", self.name, error)
            })?;

        Ok(image)
    }
}
