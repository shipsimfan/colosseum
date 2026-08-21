use crate::{
    Error, Result,
    render::frame_graph::{FrameGraphDynamicTransientResourceInfo, FrameGraphTransientResource},
};
use alexandria::{
    gpu::{
        VulkanDevice, VulkanImageLayout, VulkanImageTiling, VulkanImageType, VulkanSampleCountFlag,
        VulkanSharingMode,
    },
    math::Vector2u,
};
use std::cell::UnsafeCell;

impl FrameGraphTransientResource {
    /// Create a new [`FrameGraphTransientResource`] from dynamic information
    pub fn from_dynamic(
        info: &FrameGraphDynamicTransientResourceInfo,
        size: Vector2u,
        device: &VulkanDevice,
    ) -> Result<FrameGraphTransientResource> {
        let image = device
            .create_image(
                0,
                VulkanImageType::_2d,
                info.format(),
                size.extend(1),
                1,
                1,
                VulkanSampleCountFlag::_1,
                VulkanImageTiling::Optimal,
                info.usage(),
                VulkanSharingMode::Exclusive,
                &[],
                VulkanImageLayout::Undefined,
            )
            .map_err(Error::new_inner)?;

        let memory_requirements = image.get_memory_requirements();

        Ok(FrameGraphTransientResource {
            size,
            image,
            image_view: None,
            memory_requirements,
            aspect_mask: info.aspect_mask(),
            used: UnsafeCell::new(false),
        })
    }
}
