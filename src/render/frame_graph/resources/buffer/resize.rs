use crate::{
    Result,
    render::{GraphicsDevice, frame_graph::resources::FrameGraphTransientResourceBuffer},
};
use alexandria::{
    gpu::{VulkanMemoryPropertyFlag, VulkanMemoryRequirements},
    math::Vector2u,
};

impl FrameGraphTransientResourceBuffer {
    /// Resize the transient resource buffer to match the new size
    pub fn resize(
        &mut self,
        new_size: Vector2u,
        frame_index: usize,
        device: &GraphicsDevice,
    ) -> Result<()> {
        let size = &mut self.size[frame_index];
        let image_views = &mut self.image_views[frame_index];
        let memory = &mut self.memory[frame_index];

        // Check if a resize is actually needed
        if *size == new_size {
            return Ok(());
        }

        // Free any GPU memory being used by the current resources
        image_views.clear();
        *memory = None;

        // Create the images and image views and determine the allocation required
        let mut images = Vec::with_capacity(self.info.len());
        let mut memory_requirements = VulkanMemoryRequirements::default();
        for info in &self.info {
            let image = info.create_image(new_size, frame_index, device)?;
            let image_memory_requirements = image.get_memory_requirements();

            memory_requirements = memory_requirements.extend(&image_memory_requirements);

            images.push((image, image_memory_requirements));
        }

        if *memory_requirements.size() == 0 {
            return Ok(());
        }

        // Allocate the memory for the new resources
        let device_memory = device.allocate_memory(
            &memory_requirements,
            VulkanMemoryPropertyFlag::DeviceLocal,
            self.memory_name,
        )?;

        // Bind the allocated memory to the new resources and create the image views for them
        let mut offset = 0;
        for ((image, memory_requirements), info) in images.into_iter().zip(&self.info) {
            let (image_view, new_offset) = info.create_image_view(
                image,
                &memory_requirements,
                &device_memory,
                offset,
                frame_index,
                device,
            )?;

            offset = new_offset;
            image_views.push(image_view);
        }

        *memory = Some(device_memory);
        *size = new_size;
        Ok(())
    }
}
