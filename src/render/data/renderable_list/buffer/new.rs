use crate::{Error, Result, render::data::renderable_list::RenderableBuffer};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferUsageFlag, VulkanDevice, VulkanMemoryAllocateFlag,
    VulkanMemoryPropertyFlag, VulkanSharingMode,
};

impl<T> RenderableBuffer<T> {
    /// Create a new [`RenderableBuffer`]
    pub fn new(
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<RenderableBuffer<T>> {
        // Create the buffer
        let mut buffer = device
            .create_buffer(
                0,
                RenderableBuffer::<T>::SIZE_BYTES as _,
                VulkanBufferUsageFlag::ShaderDeviceAddress,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(Error::new_inner)?;

        // Allocate memory for the buffer
        let memory_requirements = buffer.get_memory_requirements();
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
            .ok_or(Error::new("cannot find memory for an object buffer"))?;
        let memory = device
            .allocate_memory_flags(
                memory_requirements.size(),
                memory_type_index,
                VulkanMemoryAllocateFlag::DeviceAddress,
            )
            .map_err(Error::new_inner)?;

        // Bind the buffer to the allocated memory
        buffer.bind_memory(&memory, 0).map_err(Error::new_inner)?;
        let base_address = buffer.get_device_address();

        // Map the buffer memory to a pointer
        let memory = memory
            .map(0, RenderableBuffer::<T>::SIZE_BYTES, 0)
            .map_err(|(error, _)| Error::new_inner(error))?;

        // Create a zeroed buffer for the renderables
        let renderables = Vec::with_capacity(RenderableBuffer::<T>::SIZE);

        Ok(RenderableBuffer {
            buffer,
            memory,
            base_address,
            renderables,
        })
    }
}
