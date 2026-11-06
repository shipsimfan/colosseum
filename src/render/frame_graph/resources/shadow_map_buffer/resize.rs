use crate::{
    Result,
    render::{
        DescriptorSetBinding, GraphicsDevice, MAX_FRAMES_IN_FLIGHT,
        frame_graph::resources::FrameGraphShadowMapBuffer,
    },
};
use alexandria::{
    gpu::{
        VulkanDescriptorImageInfo, VulkanDescriptorSet, VulkanImageLayout,
        VulkanMemoryPropertyFlag, VulkanWriteDescriptorSet,
    },
    math::Vector2u,
};
use colosseum_core::new_error;

impl FrameGraphShadowMapBuffer {
    /// Resize the transient resource buffer to match the new size
    pub fn resize(
        &mut self,
        new_size: u32,
        new_array_layers: u32,
        frame_index: usize,
        device: &GraphicsDevice,
        descriptor_sets: &[(
            [VulkanDescriptorSet; MAX_FRAMES_IN_FLIGHT],
            &'static [DescriptorSetBinding],
        )],
    ) -> Result<()> {
        let new_size = Vector2u::splat(new_size);
        let new_array_layers = self.info.extra_data().cascades() * new_array_layers;

        let size = &mut self.size[frame_index];
        let array_layers = &mut self.array_layers[frame_index];
        let image_views = &mut self.image_views[frame_index];
        let full_image_view = &mut self.full_image_view[frame_index];
        let memory = &mut self.memory[frame_index];

        // Check if a resize is actually needed
        if *size == new_size && *array_layers == new_array_layers {
            return Ok(());
        }

        // Free any GPU memory being used by the current resources
        image_views.clear();
        *full_image_view = None;
        *memory = None;

        if new_array_layers == 0 {
            *size = new_size;
            *array_layers = new_array_layers;
            return Ok(());
        }

        // Create the images and image views and determine the allocation required
        let mut image = self.info.create_image(
            new_size,
            new_array_layers,
            self.info.extra_data().cube_map(),
            frame_index,
            device,
        )?;
        let memory_requirements = image.get_memory_requirements();

        // Allocate the memory for the new resources
        let device_memory = device.allocate_memory(
            &memory_requirements,
            VulkanMemoryPropertyFlag::DeviceLocal,
            &self.memory_name,
        )?;

        // Bind the allocated memory to the new resource
        image.bind_memory(&device_memory, 0).map_err(|error| {
            new_error!(
                "unable to bind memory for {} shadow map buffer - {}",
                self.info.name(),
                error
            )
        })?;

        // Create the complete image view
        let new_full_image_view = self.info.create_shadow_map_full_image_view(
            &image,
            new_array_layers,
            self.info.extra_data().cube_map(),
            frame_index,
            device,
        )?;

        // Update the descriptor sets with the new image view
        for (descriptor_set, binding, sampler) in self.info.descriptor_sets() {
            device.update_descriptor_sets(
                &[VulkanWriteDescriptorSet::new(
                    &descriptor_sets[*descriptor_set].0[frame_index],
                    *binding,
                    0,
                    descriptor_sets[*descriptor_set].1[*binding as usize].r#type(),
                    &[VulkanDescriptorImageInfo::new(
                        sampler,
                        &new_full_image_view,
                        VulkanImageLayout::ShaderReadOnlyOptimal,
                    )],
                    &[],
                )],
                &[],
            );
        }

        // Create the individual layer image views
        let image_view_count = new_array_layers / self.info.extra_data().cascades();
        image_views.reserve(image_view_count as _);
        for image_view_index in 0..image_view_count {
            let layer = image_view_index * self.info.extra_data().cascades();

            // Create the image view
            image_views.push(self.info.create_shadow_map_image_view(
                &image,
                layer,
                self.info.extra_data().cube_map(),
                self.info.extra_data().cascades(),
                frame_index,
                device,
            )?);
        }

        *memory = Some(device_memory);
        *full_image_view = Some(new_full_image_view);
        *size = new_size;
        *array_layers = new_array_layers;
        Ok(())
    }
}
