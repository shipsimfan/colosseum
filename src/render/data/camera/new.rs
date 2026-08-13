use crate::{
    Error, Result,
    render::{CameraRenderData, RenderObjects, data::camera::CameraShaderData},
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferUsageFlag, VulkanDescriptorBufferInfo,
    VulkanDescriptorPoolSize, VulkanDescriptorType, VulkanDevice, VulkanMemoryPropertyFlag,
    VulkanSharingMode, VulkanWriteDescriptorSet,
};

impl CameraRenderData {
    /// Create a new set of [`CameraRenderData`]
    pub(in crate::render::data) fn new(
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
        render_objects: &RenderObjects,
    ) -> Result<CameraRenderData> {
        // Create buffer for camera shader data
        let mut buffer = device
            .create_buffer(
                0,
                std::mem::size_of::<CameraShaderData>() as _,
                VulkanBufferUsageFlag::UniformBuffer,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(Error::new_inner)?;

        // Allocate memory for camera shader data
        let memory_requirements = buffer.get_memory_requirements();
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
            .ok_or(Error::new("unable to find memory for camera buffer"))?;
        let memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;
        buffer.bind_memory(&memory, 0).map_err(Error::new_inner)?;

        let mut shader_data = memory
            .map(0, std::mem::size_of::<CameraShaderData>(), 0)
            .map_err(|(error, _)| Error::new_inner(error))?;
        shader_data[0] = CameraShaderData::new();

        // Create a descriptor pool for the camera shader data
        let mut descriptor_pool = device
            .create_descriptor_pool(
                0,
                1,
                &[VulkanDescriptorPoolSize::new(
                    VulkanDescriptorType::UniformBuffer,
                    1,
                )],
            )
            .map_err(Error::new_inner)?;

        // Allocate a descriptor set for the camera shader data
        let descriptor_set = descriptor_pool
            .allocate_descriptor_set(render_objects.camera_data_layout())
            .map_err(Error::new_inner)?;

        // Update the descriptor set with the camera shader data buffer
        device.update_descriptor_sets(
            &[VulkanWriteDescriptorSet::new(
                &descriptor_set,
                0,
                0,
                VulkanDescriptorType::UniformBuffer,
                &[],
                &[VulkanDescriptorBufferInfo::new(
                    &buffer,
                    0,
                    std::mem::size_of::<CameraShaderData>() as _,
                )],
            )],
            &[],
        );

        Ok(CameraRenderData {
            descriptor_set,
            descriptor_pool,
            buffer,
            shader_data,
        })
    }
}
