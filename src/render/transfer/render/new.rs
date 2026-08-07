use crate::{
    Error, Result,
    render::{
        RenderGpuTransferQueue,
        transfer::{GpuTransferCommand, StagingBuffer},
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanCommandBufferLevel, VulkanCommandPoolCreateFlag,
    VulkanDevice, VulkanQueue,
};
use std::sync::{Arc, mpsc::Receiver};

const INITIAL_STAGING_BUFFER_CAPACITY: usize = 64;

impl RenderGpuTransferQueue {
    /// Create a new [`RenderGpuTransferQueue`]
    pub(in crate::render::transfer) fn new(
        queue: &mut VulkanQueue,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
        device: &VulkanDevice,
        receiver: Receiver<GpuTransferCommand>,
    ) -> Result<RenderGpuTransferQueue> {
        // Create the transfer command pool and command buffer
        let mut command_pool = device
            .create_command_pool(
                queue.queue_family(),
                VulkanCommandPoolCreateFlag::ResetCommandBuffer,
            )
            .map_err(Error::new_inner)?;
        let command_buffer_id = command_pool
            .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
            .map_err(Error::new_inner)?;

        // Create the transfer fence
        let fence = device.create_fence(0).map_err(Error::new_inner)?;

        // Create staging buffers
        let vertex_staging_buffer = StagingBuffer::new(
            INITIAL_STAGING_BUFFER_CAPACITY,
            device.clone(),
            &memory_properties,
        )?;
        let index_staging_buffer = StagingBuffer::new(
            INITIAL_STAGING_BUFFER_CAPACITY,
            device.clone(),
            &memory_properties,
        )?;

        Ok(RenderGpuTransferQueue {
            receiver,
            command_pool,
            command_buffer_id,
            fence,
            vertex_staging_buffer,
            index_staging_buffer,
        })
    }
}
