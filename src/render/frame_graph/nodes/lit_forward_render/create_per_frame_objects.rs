use crate::{
    Result,
    render::{
        FixedRenderObjects, LightingData, LightingMetadata, PerFrameObjectBuilder,
        RenderDirectionalLight, RenderPointLight, RenderSpotLight,
        frame_graph::LitForwardRenderNode,
    },
};
use alexandria::{
    gpu::{VulkanBufferUsageFlag, VulkanDescriptorType},
    math::Matrix4x4f,
};

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
            VulkanDescriptorType::UniformBuffer,
            vec![(0, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into()],
            FixedRenderObjects::LIGHTING_METADATA_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderDirectionalLight, _>(
            LightingData::INITIAL_DIRECTIONAL_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![(1, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into()],
            FixedRenderObjects::DIRECTIONAL_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderPointLight, _>(
            LightingData::INITIAL_POINT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![(2, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into()],
            FixedRenderObjects::POINT_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderSpotLight, _>(
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![(3, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into()],
            FixedRenderObjects::SPOT_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<Matrix4x4f, _>(
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![
                (4, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into(),
                (0, FixedRenderObjects::SPOT_LIGHT_DESCRIPTOR_SET).into(),
            ],
            FixedRenderObjects::SPOT_LIGHT_MATRICES_DEVICE_BUFFER,
        )?;

        // Create the shadow map buffers
        per_frame_objects.add_shadow_map_buffer(
            (1024, 1024),
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            FixedRenderObjects::SPOT_LIGHT_SHADOW_MAPS,
        )?;

        Ok(())
    }
}
