use crate::{
    Result,
    render::{
        DeviceDataBuffer, FixedRenderObjects, RenderData, ShadowMapBuffer,
        frame_graph::LitForwardRenderNode,
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanDescriptorSet, VulkanDevice,
};

impl LitForwardRenderNode {
    /// Copy data from staging buffers to device local buffers
    pub(in crate::render::frame_graph) fn copy_data(
        render_data: &RenderData,
        device_buffers: &mut [DeviceDataBuffer],
        shadow_maps: &mut [ShadowMapBuffer],
        descriptor_sets: &[VulkanDescriptorSet],
        fixed_objects: &FixedRenderObjects,
        cmd_buffer: &mut VulkanCommandBuffer,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        // Copy lighting data device buffers
        device_buffers[FixedRenderObjects::LIGHTING_METADATA_DEVICE_BUFFER].copy(
            render_data.lighting().metadata(),
            descriptor_sets,
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::DIRECTIONAL_LIGHTS_DEVICE_BUFFER].copy(
            render_data.lighting().directional_lights(),
            descriptor_sets,
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::POINT_LIGHTS_DEVICE_BUFFER].copy(
            render_data.lighting().point_lights(),
            descriptor_sets,
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::SPOT_LIGHTS_DEVICE_BUFFER].copy(
            render_data.lighting().spot_lights(),
            descriptor_sets,
            cmd_buffer,
            device,
            memory_properties,
        )?;

        // Copy lighting matrix device buffers
        device_buffers[FixedRenderObjects::DIRECTIONAL_LIGHT_MATRICES_DEVICE_BUFFER].copy(
            render_data.lighting().directional_light_matrices(),
            descriptor_sets,
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::SPOT_LIGHT_MATRICES_DEVICE_BUFFER].copy(
            render_data.lighting().spot_light_matrices(),
            descriptor_sets,
            cmd_buffer,
            device,
            memory_properties,
        )?;

        // Resize shadow maps
        shadow_maps[FixedRenderObjects::DIRECTIONAL_LIGHT_SHADOW_MAPS].reserve(
            render_data.lighting().directional_lights().capacity(),
            descriptor_sets,
            fixed_objects.sampler(FixedRenderObjects::LINEAR_CLAMP_SAMPLER),
            device,
            memory_properties,
        )?;
        shadow_maps[FixedRenderObjects::SPOT_LIGHT_SHADOW_MAPS].reserve(
            render_data.lighting().spot_lights().capacity(),
            descriptor_sets,
            fixed_objects.sampler(FixedRenderObjects::LINEAR_CLAMP_SAMPLER),
            device,
            memory_properties,
        )
    }
}
