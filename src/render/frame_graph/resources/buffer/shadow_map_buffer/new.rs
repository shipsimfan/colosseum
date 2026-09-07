use crate::{Error, Result, render::ShadowMapBuffer};
use alexandria::{
    gpu::{
        VulkanAdapterMemoryProperties, VulkanComponentMapping, VulkanDevice, VulkanImageAspectFlag,
        VulkanImageCreateFlag, VulkanImageLayout, VulkanImageTiling, VulkanImageType,
        VulkanImageUsageFlag, VulkanImageViewType, VulkanMemoryPropertyFlag, VulkanSampleCountFlag,
        VulkanSharingMode,
    },
    math::Vector2u,
};

impl ShadowMapBuffer {
    /// Create a new [`ShadowMapBuffer`]
    pub(in crate::render::frame_graph::resources::buffer) fn new(
        size: Vector2u,
        count: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<ShadowMapBuffer> {
        // Create the image
        let mut image = device
            .create_image(
                VulkanImageCreateFlag::_2dArrayCompatible,
                VulkanImageType::_3d,
                ShadowMapBuffer::FORMAT,
                size.extend(count as _),
                1,
                1,
                VulkanSampleCountFlag::_1,
                VulkanImageTiling::Optimal,
                VulkanImageUsageFlag::DepthStencilAttachment | VulkanImageUsageFlag::Sampled,
                VulkanSharingMode::Exclusive,
                &[],
                VulkanImageLayout::Undefined,
            )
            .map_err(Error::new_inner)?;

        // Allocate and bind the memory for the image
        let memory_requirements = image.get_memory_requirements();
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::DeviceLocal,
            )
            .ok_or(Error::new("unable to find memory for a shadow map buffer"))?;

        let memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;

        image.bind_memory(&memory, 0).map_err(Error::new_inner)?;

        // Create the complete image view
        let complete_image_view = image
            .create_image_view(
                0,
                VulkanImageViewType::_2dArray,
                ShadowMapBuffer::FORMAT,
                VulkanComponentMapping::default(),
                VulkanImageAspectFlag::Depth,
                0,
                1,
                0,
                count as _,
            )
            .map_err(Error::new_inner)?;

        // Create the individual image views
        let mut layer_image_views = Vec::with_capacity(count);
        for i in 0..count {
            layer_image_views.push(
                image
                    .create_image_view(
                        0,
                        VulkanImageViewType::_2d,
                        ShadowMapBuffer::FORMAT,
                        VulkanComponentMapping::default(),
                        VulkanImageAspectFlag::Depth,
                        0,
                        1,
                        i as _,
                        1,
                    )
                    .map_err(Error::new_inner)?,
            );
        }

        Ok(ShadowMapBuffer {
            image,
            memory,
            complete_image_view,
            layer_image_views,
            size,
        })
    }
}
