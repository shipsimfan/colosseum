use crate::{
    Result,
    render::{
        FixedRenderObjects, LightingData, LightingMetadata, PerFrameObjectBuilder,
        RenderDirectionalLight, RenderPointLight, RenderSpotLight,
        frame_graph::LitForwardRenderNode,
    },
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanDescriptorType};

impl LitForwardRenderNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        // Create the lighting descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
        )?;

        // Create the lighting device data buffers
        per_frame_objects.add_device_data_buffer::<LightingMetadata, _>(
            1,
            VulkanBufferUsageFlag::UniformBuffer,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
            VulkanDescriptorType::UniformBuffer,
            0,
            FixedRenderObjects::LIGHTING_METADATA_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderDirectionalLight, _>(
            LightingData::INITIAL_DIRECTIONAL_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
            VulkanDescriptorType::StorageBuffer,
            1,
            FixedRenderObjects::DIRECTIONAL_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderPointLight, _>(
            LightingData::INITIAL_POINT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
            VulkanDescriptorType::StorageBuffer,
            2,
            FixedRenderObjects::POINT_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderSpotLight, _>(
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
            VulkanDescriptorType::StorageBuffer,
            3,
            FixedRenderObjects::SPOT_LIGHTS_DEVICE_BUFFER,
        )?;

        Ok(())
    }
}
