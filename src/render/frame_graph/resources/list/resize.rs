use crate::{
    Error, Result,
    render::frame_graph::{
        FrameGraphDynamicTransientResourceInfo, FrameGraphTransientResource,
        resources::FrameGraphResourceList,
    },
};
use alexandria::{
    gpu::{
        VulkanAdapterMemoryProperties, VulkanDevice, VulkanMemoryPropertyFlag,
        VulkanMemoryRequirements,
    },
    math::Vector2u,
};

impl FrameGraphResourceList {
    pub fn resize(
        &mut self,
        info: &[FrameGraphDynamicTransientResourceInfo],
        size: Vector2u,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        // Clear the transient resources
        self.resources.clear();
        self.memory = None;

        // Create the new images and image views for the transient resources at the new size
        // and calculate the memory requirements
        let mut memory_requirements = VulkanMemoryRequirements::default();
        for info in info {
            let resource = FrameGraphTransientResource::from_dynamic(info, size, device)?;
            memory_requirements = memory_requirements.extend(resource.memory_requirements());
            self.resources.push(resource);
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
        if *memory_requirements.size() == 0 {
            return Ok(());
        }

        let memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;

        // Bind the required memory to the new transient resources
        let mut offset = 0;
        for (resource, info) in self.resources.iter_mut().zip(info) {
            offset = resource.bind_memory(info, &memory, offset)?;
        }

        self.memory = Some(memory);

        Ok(())
    }
}
