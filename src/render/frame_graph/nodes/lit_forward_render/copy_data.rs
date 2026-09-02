use crate::{
    Result,
    render::{DeviceDataBuffer, FixedRenderObjects, RenderData, frame_graph::LitForwardRenderNode},
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanDescriptorSet, VulkanDevice,
};

impl LitForwardRenderNode {
    /// Copy data from staging buffers to device local buffers
    pub(in crate::render::frame_graph) fn copy_data(
        render_data: &RenderData,
        device_buffers: &mut [DeviceDataBuffer],
        descriptor_sets: &[VulkanDescriptorSet],
        cmd_buffer: &mut VulkanCommandBuffer,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        device_buffers[FixedRenderObjects::LIGHTING_METADATA_DEVICE_BUFFER].copy(
            render_data.lighting().metadata(),
            &descriptor_sets[FixedRenderObjects::LIGHTING_DESCRIPTOR_SET],
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::DIRECTIONAL_LIGHTS_DEVICE_BUFFER].copy(
            render_data.lighting().directional_lights(),
            &descriptor_sets[FixedRenderObjects::LIGHTING_DESCRIPTOR_SET],
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::POINT_LIGHTS_DEVICE_BUFFER].copy(
            render_data.lighting().point_lights(),
            &descriptor_sets[FixedRenderObjects::LIGHTING_DESCRIPTOR_SET],
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::SPOT_LIGHTS_DEVICE_BUFFER].copy(
            render_data.lighting().spot_lights(),
            &descriptor_sets[FixedRenderObjects::LIGHTING_DESCRIPTOR_SET],
            cmd_buffer,
            device,
            memory_properties,
        )
    }
}
