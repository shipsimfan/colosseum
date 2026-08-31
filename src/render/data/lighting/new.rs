use crate::{
    Error, Result,
    render::{
        FixedRenderObjects, LightingData, RenderDirectionalLight, RenderObjects, RenderPointLight,
        RenderSpotLight,
        data::lighting::{LightingDataBuffer, LightingMetadata},
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferUsageFlag, VulkanDescriptorBufferInfo,
    VulkanDescriptorPool, VulkanDescriptorType, VulkanDevice, VulkanMemoryPropertyFlag,
    VulkanSharingMode, VulkanWriteDescriptorSet,
};

const INITIAL_DIRECTIONAL_LIGHT_CAPACITY: usize = 1;
const INITIAL_POINT_LIGHT_CAPACITY: usize = 32;
const INITIAL_SPOT_LIGHT_CAPACITY: usize = 8;

impl LightingData {
    /// Create a new set of [`LightingData`]
    pub(in crate::render::data) fn new(
        descriptor_pool: &mut VulkanDescriptorPool,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
        render_objects: &RenderObjects,
    ) -> Result<LightingData> {
        // Create buffer for lighting metadata
        let mut metadata_buffer = device
            .create_buffer(
                0,
                std::mem::size_of::<LightingMetadata>() as _,
                VulkanBufferUsageFlag::UniformBuffer,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(Error::new_inner)?;

        // Allocate memory for lighting metadata
        let memory_requirements = metadata_buffer.get_memory_requirements();
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
            .ok_or(Error::new(
                "unable to find memory for lighting metadata buffer",
            ))?;
        let metadata_memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;
        metadata_buffer
            .bind_memory(&metadata_memory, 0)
            .map_err(Error::new_inner)?;

        let mut metadata = metadata_memory
            .map(0, std::mem::size_of::<LightingMetadata>(), 0)
            .map_err(|(error, _)| Error::new_inner(error))?;
        metadata[0] = LightingMetadata::default();

        // Allocate a descriptor set for the lighting data
        let descriptor_set = descriptor_pool
            .allocate_descriptor_set(
                render_objects
                    .descriptor_set_layout(FixedRenderObjects::LIGHTING_DESCRIPTOR_SET_LAYOUT),
            )
            .map_err(Error::new_inner)?;

        // Create the light data buffers
        let directional_lights = LightingDataBuffer::new(
            INITIAL_DIRECTIONAL_LIGHT_CAPACITY,
            device,
            memory_properties,
        )?;

        let point_lights =
            LightingDataBuffer::new(INITIAL_POINT_LIGHT_CAPACITY, device, memory_properties)?;

        let spot_lights =
            LightingDataBuffer::new(INITIAL_SPOT_LIGHT_CAPACITY, device, memory_properties)?;

        // Update the descriptor set with the buffers
        device.update_descriptor_sets(
            &[
                VulkanWriteDescriptorSet::new(
                    &descriptor_set,
                    0,
                    0,
                    VulkanDescriptorType::UniformBuffer,
                    &[],
                    &[VulkanDescriptorBufferInfo::new(
                        &metadata_buffer,
                        0,
                        std::mem::size_of::<LightingMetadata>() as _,
                    )],
                ),
                VulkanWriteDescriptorSet::new(
                    &descriptor_set,
                    1,
                    0,
                    VulkanDescriptorType::StorageBuffer,
                    &[],
                    &[
                        VulkanDescriptorBufferInfo::new(
                            directional_lights.buffer(),
                            0,
                            (std::mem::size_of::<RenderDirectionalLight>()
                                * directional_lights.capacity()) as _,
                        ),
                        VulkanDescriptorBufferInfo::new(
                            point_lights.buffer(),
                            0,
                            (std::mem::size_of::<RenderPointLight>() * point_lights.capacity())
                                as _,
                        ),
                        VulkanDescriptorBufferInfo::new(
                            spot_lights.buffer(),
                            0,
                            (std::mem::size_of::<RenderSpotLight>() * spot_lights.capacity()) as _,
                        ),
                    ],
                ),
            ],
            &[],
        );

        Ok(LightingData {
            descriptor_set,
            metadata_buffer,
            metadata,
            directional_lights,
            point_lights,
            spot_lights,
        })
    }
}
