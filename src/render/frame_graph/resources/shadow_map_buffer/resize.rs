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

impl FrameGraphShadowMapBuffer {
    /// Resize the transient resource buffer to match the new size
    pub fn resize(
        &mut self,
        new_size: Vector2u,
        new_array_layers: u32,
        frame_index: usize,
        device: &GraphicsDevice,
        descriptor_sets: &[(
            [VulkanDescriptorSet; MAX_FRAMES_IN_FLIGHT],
            &'static [DescriptorSetBinding],
        )],
    ) -> Result<()> {
        let size = &mut self.size[frame_index];
        let array_layers = &mut self.array_layers[frame_index];
        let image_views = &mut self.image_views[frame_index];
        let memory = &mut self.memory[frame_index];

        // Check if a resize is actually needed
        if *size == new_size && *array_layers == new_array_layers {
            return Ok(());
        }

        // Free any GPU memory being used by the current resources
        image_views.clear();
        *memory = None;

        if new_array_layers == 0 {
            *size = new_size;
            *array_layers = new_array_layers;
            return Ok(());
        }

        // Create the images and image views and determine the allocation required
        let image = self
            .info
            .create_image(new_size, new_array_layers, frame_index, device)?;
        let memory_requirements = image.get_memory_requirements();

        // Allocate the memory for the new resources
        let device_memory = device.allocate_memory(
            &memory_requirements,
            VulkanMemoryPropertyFlag::DeviceLocal,
            self.memory_name,
        )?;

        // Bind the allocated memory to the new resources, create the image views for them, and
        // update descriptor sets
        let mut offset = 0;
        image_views.reserve(new_array_layers as _);
        for _ in 0..new_array_layers {
            // Create the image view
            let (image_view, new_offset) = self.info.create_image_view(
                &image,
                &memory_requirements,
                &device_memory,
                offset,
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
                            &image_view,
                            VulkanImageLayout::ShaderReadOnlyOptimal,
                        )],
                        &[],
                    )],
                    &[],
                );
            }

            // Update the offset and image views
            offset = new_offset;
            image_views.push(image_view);
        }

        *memory = Some(device_memory);
        *size = new_size;
        *array_layers = new_array_layers;
        Ok(())
    }
}
