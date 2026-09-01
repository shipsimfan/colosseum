use crate::{
    Result,
    render::{DeviceDataBuffer, RenderData, frame_graph::LitForwardRenderNode},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanDevice};

impl LitForwardRenderNode {
    /// Copy data from staging buffers to device local buffers
    pub(in crate::render::frame_graph) fn copy_data(
        render_data: &RenderData,
        data_buffers: &mut [DeviceDataBuffer],
        cmd_buffer: &mut VulkanCommandBuffer,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        todo!()
    }
}
