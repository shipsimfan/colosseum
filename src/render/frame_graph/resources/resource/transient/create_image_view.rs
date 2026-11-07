#[cfg(debug_assertions)]
use std::ffi::CString;

use crate::{
    Result,
    render::{GraphicsDevice, frame_graph::FrameGraphTransientResourceInfo},
};
use alexandria::gpu::{
    VulkanComponentMapping, VulkanDeviceMemory, VulkanImage, VulkanImageView, VulkanImageViewType,
    VulkanMemoryRequirements,
};
use colosseum_core::new_error;

impl<ExtraData> FrameGraphTransientResourceInfo<ExtraData> {
    /// Create a new image view for this resource
    pub fn create_image_view(
        &self,
        mut image: VulkanImage,
        memory_requirements: &VulkanMemoryRequirements,
        memory: &VulkanDeviceMemory,
        mut offset: u64,
        index: usize,
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
            .set_object_name(
                &mut image_view,
                &CString::new(format!("{}-{}", self.image_view_name[frame_index], index)).unwrap(),
            )
            .map_err(|error| {
                new_error!(
                    "unable to set image view name for \"{}\" - {}",
                    self.name,
                    error
                )
            })?;

        Ok((image_view, offset + *memory_requirements.size()))
    }

    /// Create a full image view for a shadow map resource
    pub fn create_shadow_map_full_image_view(
        &self,
        image: &VulkanImage,
        array_layers: u32,
        cube_map: bool,
        frame_index: usize,
        device: &GraphicsDevice,
    ) -> Result<VulkanImageView> {
        // Create the image view
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut image_view = image
            .create_image_view(
                0,
                if cube_map {
                    VulkanImageViewType::CubeArray
                } else {
                    VulkanImageViewType::_2dArray
                },
                self.format,
                VulkanComponentMapping::default(),
                self.aspect_mask(),
                0,
                1,
                0,
                array_layers,
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
            .set_object_name(
                &mut image_view,
                &CString::new(format!("Full {}", self.image_view_name[frame_index])).unwrap(),
            )
            .map_err(|error| {
                new_error!(
                    "unable to set image view name for \"{}\" - {}",
                    self.name,
                    error
                )
            })?;

        Ok(image_view)
    }

    /// Create a image view for a single layer in a shadow map resource
    pub fn create_shadow_map_image_view(
        &self,
        image: &VulkanImage,
        layer: u32,
        cube_map: bool,
        cascades: u32,
        index: usize,
        frame_index: usize,
        device: &GraphicsDevice,
    ) -> Result<VulkanImageView> {
        // Create the image view
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut image_view = image
            .create_image_view(
                0,
                if cube_map {
                    VulkanImageViewType::Cube
                } else if cascades > 1 {
                    VulkanImageViewType::_2dArray
                } else {
                    VulkanImageViewType::_2d
                },
                self.format,
                VulkanComponentMapping::default(),
                self.aspect_mask(),
                0,
                1,
                layer,
                cascades,
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
            .set_object_name(
                &mut image_view,
                &CString::new(format!("{}-{}", self.image_view_name[frame_index], index)).unwrap(),
            )
            .map_err(|error| {
                new_error!(
                    "unable to set image view name for \"{}\" - {}",
                    self.name,
                    error
                )
            })?;

        Ok(image_view)
    }
}
