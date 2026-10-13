use crate::{
    Result,
    render::{
        FixedRenderObjects, LightingData, LightingMetadata, PerFrameObjectBuilder,
        RenderDirectionalLight, RenderPointLight, RenderSpotLight,
        frame_graph::{LitForwardRenderNode, ShadowMapLight},
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
        index: usize,
    ) -> Result<()> {
        // Create the lighting descriptor set
        per_frame_objects.add_descriptor_set(
            format!("Lighting Descriptor Set {}", index),
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
        )?;

        // Create the lighting device data buffers
        per_frame_objects.add_device_data_buffer::<LightingMetadata, _>(
            format!("Lighting Metadata Buffer {}", index),
            1,
            VulkanBufferUsageFlag::UniformBuffer,
            VulkanDescriptorType::UniformBuffer,
            vec![(0, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into()],
            FixedRenderObjects::LIGHTING_METADATA_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderDirectionalLight, _>(
            format!("Directional Light Buffer {}", index),
            LightingData::INITIAL_DIRECTIONAL_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![(1, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into()],
            FixedRenderObjects::DIRECTIONAL_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<Matrix4x4f, _>(
            format!("Directional Light Matrix Buffer {}", index),
            LightingData::INITIAL_DIRECTIONAL_LIGHT_CAPACITY * 4,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![
                (2, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into(),
                (0, FixedRenderObjects::DIRECTIONAL_LIGHT_DESCRIPTOR_SET).into(),
            ],
            FixedRenderObjects::DIRECTIONAL_LIGHT_MATRICES_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderPointLight, _>(
            format!("Point Light Buffer {}", index),
            LightingData::INITIAL_POINT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![
                (4, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into(),
                (2, FixedRenderObjects::POINT_LIGHT_DESCRIPTOR_SET).into(),
            ],
            FixedRenderObjects::POINT_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<Matrix4x4f, _>(
            format!("Point Light Matrix Buffer {}", index),
            LightingData::INITIAL_POINT_LIGHT_CAPACITY * 6,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![
                (5, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into(),
                (0, FixedRenderObjects::POINT_LIGHT_DESCRIPTOR_SET).into(),
            ],
            FixedRenderObjects::POINT_LIGHT_MATRICES_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<RenderSpotLight, _>(
            format!("Spot Light Buffer {}", index),
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![(7, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into()],
            FixedRenderObjects::SPOT_LIGHTS_DEVICE_BUFFER,
        )?;
        per_frame_objects.add_device_data_buffer::<Matrix4x4f, _>(
            format!("Spot Light Matrix Buffer {}", index),
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            VulkanBufferUsageFlag::StorageBuffer,
            VulkanDescriptorType::StorageBuffer,
            vec![
                (8, FixedRenderObjects::LIGHTING_DESCRIPTOR_SET).into(),
                (0, FixedRenderObjects::SPOT_LIGHT_DESCRIPTOR_SET).into(),
            ],
            FixedRenderObjects::SPOT_LIGHT_MATRICES_DEVICE_BUFFER,
        )?;

        // Create the shadow map buffers
        let shadow_quality = per_frame_objects.shadow_quality() as usize;
        per_frame_objects.add_shadow_map_buffer(
            format!("Directional Light Shadow Map Buffer {}", index),
            RenderDirectionalLight::SIZE[shadow_quality],
            LightingData::INITIAL_DIRECTIONAL_LIGHT_CAPACITY,
            RenderDirectionalLight::CASCADES,
            false,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
            3,
            FixedRenderObjects::DIRECTIONAL_LIGHT_SHADOW_MAPS,
        )?;
        per_frame_objects.add_shadow_map_buffer(
            format!("Point Light Shadow Map Buffer {}", index),
            RenderPointLight::SIZE[shadow_quality],
            LightingData::INITIAL_POINT_LIGHT_CAPACITY,
            RenderPointLight::CASCADES,
            true,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
            6,
            FixedRenderObjects::POINT_LIGHT_SHADOW_MAPS,
        )?;
        per_frame_objects.add_shadow_map_buffer(
            format!("Spot Light Shadow Map Buffer {}", index),
            RenderSpotLight::SIZE[shadow_quality],
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            RenderSpotLight::CASCADES,
            false,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
            9,
            FixedRenderObjects::SPOT_LIGHT_SHADOW_MAPS,
        )?;

        Ok(())
    }
}
