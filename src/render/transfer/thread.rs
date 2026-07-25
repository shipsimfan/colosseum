use crate::{
    Error, GlobalSharedState, Result, debug,
    logging::Logger,
    render::{
        GpuTransferQueue, Vertex,
        transfer::{GpuTransferCommand, StagingBuffer},
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBuffer, VulkanBufferCopy, VulkanCommandBuffer,
    VulkanCommandBufferLevel, VulkanCommandBufferSubmitInfo, VulkanCommandPoolCreateFlag,
    VulkanDevice, VulkanFence, VulkanQueue, VulkanSubmitInfo,
};
use std::sync::{Arc, mpsc::Receiver};

const INITIAL_STAGING_BUFFER_CAPACITY: usize = 64;

impl GpuTransferQueue {
    pub(in crate::render::transfer) fn thread(
        shared_state: &GlobalSharedState,
        receiver: Receiver<GpuTransferCommand>,
        device: VulkanDevice,
        mut queue: VulkanQueue,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
        logger: Logger,
    ) -> Result<()> {
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
        let command_buffer = &mut command_pool[command_buffer_id];

        // Create the transfer fence
        let mut fence = device.create_fence(0).map_err(Error::new_inner)?;

        // Create staging buffers
        let mut vertex_staging_buffer = StagingBuffer::new(
            INITIAL_STAGING_BUFFER_CAPACITY,
            device.clone(),
            &memory_properties,
        )?;
        let mut index_staging_buffer = StagingBuffer::new(
            INITIAL_STAGING_BUFFER_CAPACITY,
            device.clone(),
            &memory_properties,
        )?;

        while shared_state.is_running() {
            let command = match receiver.recv() {
                Ok(command) => command,
                Err(_) => {
                    debug!(&logger, "Transfer thread exiting due to channel closure");
                    break;
                }
            };

            match command {
                GpuTransferCommand::Mesh {
                    mesh,
                    render_mesh,
                    allocation,
                    sender,
                } => {
                    let vertex_staging_buffer = vertex_staging_buffer.set(mesh.vertices())?;
                    let index_staging_buffer = index_staging_buffer.set(mesh.indices())?;

                    copy_buffers(
                        command_buffer,
                        &mut queue,
                        &mut fence,
                        &[
                            (
                                vertex_staging_buffer,
                                render_mesh.vertex_buffer(),
                                0,
                                (mesh.vertices().len() * std::mem::size_of::<Vertex>()) as u64,
                            ),
                            (
                                index_staging_buffer,
                                render_mesh.index_buffer(),
                                0,
                                (mesh.indices().len() * std::mem::size_of::<u32>()) as u64,
                            ),
                        ],
                    )?;

                    sender.send((mesh, render_mesh, allocation))?;
                }
            }
        }

        debug!(&logger, "Transfer thread exiting");

        Ok(())
    }
}

fn copy_buffers(
    command_buffer: &mut VulkanCommandBuffer,
    queue: &mut VulkanQueue,
    fence: &mut VulkanFence,
    buffers: &[(&VulkanBuffer, &VulkanBuffer, u32, u64)],
) -> Result<()> {
    // Recored the copy commands
    command_buffer.begin().map_err(Error::new_inner)?;
    for (src, dst, offset, size) in buffers {
        command_buffer.cmd_copy_buffer(src, dst, &[VulkanBufferCopy::new(0, *offset as _, *size)]);
    }
    command_buffer.end().map_err(Error::new_inner)?;

    // Submit the copy commands
    queue
        .submit(
            &[VulkanSubmitInfo::new(
                0,
                &[],
                &[VulkanCommandBufferSubmitInfo::new(command_buffer, 0)],
                &[],
            )],
            Some(fence),
        )
        .map_err(Error::new_inner)?;

    // Wait for the copy commands to complete
    fence.wait(u64::MAX).map_err(Error::new_inner)?;

    Ok(())
}
