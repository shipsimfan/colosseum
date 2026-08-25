use crate::{
    Error, Result,
    render::frame_graph::{
        FrameGraphDynamicTransientResourceInfo, FrameGraphResources, FrameGraphTransientResource,
    },
};
use alexandria::{
    gpu::{
        VulkanAdapterMemoryProperties, VulkanDevice, VulkanMemoryPropertyFlag,
        VulkanMemoryRequirements,
    },
    math::Vector2u,
};

impl<'a> FrameGraphResources<'a> {
    /// Resize the resources to a new swapchain size
    pub fn resize(
        &mut self,
        transient_render_scale_info: &[FrameGraphDynamicTransientResourceInfo],
        swapchain_size: Vector2u,
        render_scale: f32,
        memory_properties: &VulkanAdapterMemoryProperties,
        device: &VulkanDevice,
        new_epoch: u64,
    ) -> Result<()> {
        // Clear the transient resources
        self.transient_render_scale.clear();
        *self.transient_render_scale_memory = None;

        // Compute the new render scale size
        let render_scale_size: Vector2u = (swapchain_size.into_f32() * render_scale).from_f32();

        // Create the new images and image views for the transient resources at the new size
        // and calculate the memory requirements
        let mut memory_requirements = VulkanMemoryRequirements::default();
        for info in transient_render_scale_info {
            let resource =
                FrameGraphTransientResource::from_dynamic(info, render_scale_size, device)?;
            memory_requirements = memory_requirements.extend(resource.memory_requirements());
            self.transient_render_scale.push(resource);
        }

        // Allocate a new block of memory for the transient resources
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::DeviceLocal,
            )
            .ok_or(Error::new(
                "unable to find memory for transient frame graph textures",
            ))?;
        let memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;

        // Bind the required memory to the new transient resources
        let mut offset = 0;
        for (resource, info) in self
            .transient_render_scale
            .iter_mut()
            .zip(transient_render_scale_info)
        {
            offset = resource.bind_memory(info, &memory, offset)?;
        }

        *self.transient_render_scale_memory = Some(memory);
        *self.epoch = new_epoch;

        Ok(())
    }
}
