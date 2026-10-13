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
#[cfg(debug_assertions)]
use std::ffi::CString;
use std::sync::{Arc, mpsc::Receiver};

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
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut command_pool, c"Transfer Command Pool")
            .map_err(Error::new_inner)?;

        let command_buffer_id = command_pool
            .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut command_pool[command_buffer_id],
                c"Transfer Command Buffer",
            )
            .map_err(Error::new_inner)?;

        // Create the transfer fence
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut fence = device.create_fence(0).map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut fence,
                &CString::new(format!("Transfer Fence")).unwrap(),
            )
            .map_err(Error::new_inner)?;

        // Create staging buffers
        let staging_buffer = StagingBuffer::new(
            "General Staging Buffer",
            32 * 1024, // 32 KB
            device.clone(),
            &memory_properties,
        )?;

        Ok(RenderGpuTransferQueue {
            receiver,
            command_pool,
            command_buffer_id,
            fence,
            staging_buffer,
        })
    }
}
