use crate::{
    Error, Result,
    render::{
        FrameGraphTransientBuffer, RenderData, RenderObjects, job::graphics_device::PerFrameData,
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanCommandBufferLevel, VulkanCommandPool, VulkanDevice,
};
use std::sync::Arc;

impl PerFrameData {
    /// Create a new set of [`PerFrameData`]
    pub fn new(
        render_objects: &RenderObjects,
        command_pool: &mut VulkanCommandPool,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
        device: &VulkanDevice,
    ) -> Result<PerFrameData> {
        let command_buffer = command_pool
            .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
            .map_err(Error::new_inner)?;
        let transient_buffer = FrameGraphTransientBuffer::new();
        let render_data = RenderData::new(&device, memory_properties, &render_objects)?;

        Ok(PerFrameData {
            command_buffer,
            transient_buffer,
            render_data,
        })
    }
}
