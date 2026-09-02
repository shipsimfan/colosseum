use crate::{
    Result,
    render::{
        DeviceDataBuffer, FixedRenderObjects, RenderData, frame_graph::UnlitForwardRenderNode,
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanDescriptorSet, VulkanDevice,
};

impl UnlitForwardRenderNode {
    /// Copy data from staging buffers to device local buffers
    pub(in crate::render::frame_graph) fn copy_data(
        render_data: &RenderData,
        device_buffers: &mut [DeviceDataBuffer],
        descriptor_sets: &[VulkanDescriptorSet],
        cmd_buffer: &mut VulkanCommandBuffer,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        device_buffers[FixedRenderObjects::CAMERA_DEVICE_BUFFER].copy(
            render_data.camera(),
            &descriptor_sets[FixedRenderObjects::CAMERA_DESCRIPTOR_SET],
            cmd_buffer,
            device,
            memory_properties,
        )?;
        device_buffers[FixedRenderObjects::RENDERABLES_DEVICE_BUFFER].copy(
            render_data.renderables(),
            &descriptor_sets[FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET],
            cmd_buffer,
            device,
            memory_properties,
        )
    }
}
