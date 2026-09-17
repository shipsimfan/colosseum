use crate::{Error, Result, render::ShadowMapBuffer};
use alexandria::{
    gpu::{
        VulkanAdapterMemoryProperties, VulkanComponentMapping, VulkanDescriptorImageInfo,
        VulkanDescriptorSet, VulkanDescriptorType, VulkanDevice, VulkanImageAspectFlag,
        VulkanImageCreateFlag, VulkanImageCreateFlags, VulkanImageLayout, VulkanImageTiling,
        VulkanImageType, VulkanImageUsageFlag, VulkanImageViewType, VulkanMemoryPropertyFlag,
        VulkanSampleCountFlag, VulkanSampler, VulkanSharingMode, VulkanWriteDescriptorSet,
    },
    math::Vector2u,
};
use std::ffi::CString;

impl ShadowMapBuffer {
    /// Create a new [`ShadowMapBuffer`]
    pub(in crate::render::frame_graph::resources::buffer) fn new(
        name: String,
        size: Vector2u,
        count: usize,
        cascades: usize,
        cube: bool,
        descriptor_set: usize,
        binding: u32,
        descriptor_sets: &[VulkanDescriptorSet],
        sampler: &VulkanSampler,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<ShadowMapBuffer> {
        let name = CString::new(name).unwrap();
        ShadowMapBuffer::new_inner(
            name,
            size,
            count,
            cascades,
            cube,
            descriptor_set,
            binding,
            descriptor_sets,
            sampler,
            device,
            memory_properties,
        )
    }

    /// Create a new [`ShadowMapBuffer`]
    pub(in crate::render::frame_graph::resources::buffer) fn new_inner(
        name: CString,
        size: Vector2u,
        count: usize,
        cascades: usize,
        cube: bool,
        descriptor_set: usize,
        binding: u32,
        descriptor_sets: &[VulkanDescriptorSet],
        sampler: &VulkanSampler,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<ShadowMapBuffer> {
        // Create the image
        let mut image = device
            .create_image(
                if cube {
                    VulkanImageCreateFlag::CubeCompatible.into()
                } else {
                    VulkanImageCreateFlags::empty()
                },
                VulkanImageType::_2d,
                ShadowMapBuffer::FORMAT,
                size.extend(1),
                1,
                (count * cascades) as _,
                VulkanSampleCountFlag::_1,
                VulkanImageTiling::Optimal,
                VulkanImageUsageFlag::DepthStencilAttachment | VulkanImageUsageFlag::Sampled,
                VulkanSharingMode::Exclusive,
                &[],
                VulkanImageLayout::Undefined,
            )
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut image, &name)
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
                if cube {
                    VulkanImageViewType::CubeArray
                } else {
                    VulkanImageViewType::_2dArray
                },
                ShadowMapBuffer::FORMAT,
                VulkanComponentMapping::default(),
                VulkanImageAspectFlag::Depth,
                0,
                1,
                0,
                (count * cascades) as _,
            )
            .map_err(Error::new_inner)?;

        // Create the individual image views
        let view_type = if cube {
            VulkanImageViewType::Cube
        } else if cascades > 1 {
            VulkanImageViewType::_2dArray
        } else {
            VulkanImageViewType::_2d
        };

        let mut layer_image_views = Vec::with_capacity(count);
        for i in 0..count {
            layer_image_views.push(
                image
                    .create_image_view(
                        0,
                        view_type,
                        ShadowMapBuffer::FORMAT,
                        VulkanComponentMapping::default(),
                        VulkanImageAspectFlag::Depth,
                        0,
                        1,
                        (i * cascades) as _,
                        cascades as _,
                    )
                    .map_err(Error::new_inner)?,
            );
        }

        // Bind the complete image view to the descriptor set
        device.update_descriptor_sets(
            &[VulkanWriteDescriptorSet::new(
                &descriptor_sets[descriptor_set],
                binding,
                0,
                VulkanDescriptorType::CombinedImageSampler,
                &[VulkanDescriptorImageInfo::new(
                    sampler,
                    &complete_image_view,
                    VulkanImageLayout::ShaderReadOnlyOptimal,
                )],
                &[],
            )],
            &[],
        );

        Ok(ShadowMapBuffer {
            name,
            image,
            memory,
            complete_image_view,
            layer_image_views,
            size,
            cascades,
            cube,
            descriptor_set,
            binding,
        })
    }
}
