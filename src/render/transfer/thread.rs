use crate::{
    Error, GlobalSharedState, Result,
    render::{
        CreatedRenderObject, GpuTransferQueue, Vertex,
        transfer::{GpuTransferCommand, StagingBuffer},
    },
};
use alexandria::gpu::{
    VulkanBuffer, VulkanBufferCopy, VulkanCommandBuffer, VulkanCommandBufferLevel,
    VulkanCommandBufferSubmitInfo, VulkanCommandPoolCreateFlag, VulkanDevice, VulkanFence,
    VulkanQueue, VulkanSubmitInfo,
};
use std::sync::mpsc::{Receiver, Sender};

const INITIAL_STAGING_BUFFER_CAPACITY: usize = 64;

impl GpuTransferQueue {
    pub(in crate::render::transfer) fn thread(
        shared_state: &GlobalSharedState,
        receiver: Receiver<GpuTransferCommand>,
        device: VulkanDevice,
        mut queue: VulkanQueue,
        staging_memory_type: usize,
        created_objects: Sender<CreatedRenderObject>,
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
            staging_memory_type,
        )?;
        let mut index_staging_buffer = StagingBuffer::new(
            INITIAL_STAGING_BUFFER_CAPACITY,
            device.clone(),
            staging_memory_type,
        )?;

        while shared_state.is_running() {
            let command = match receiver.recv() {
                Ok(command) => command,
                Err(_) => break,
            };

            match command {
                GpuTransferCommand::Mesh {
                    mesh,
                    shared_state,
                    render_mesh,
                } => {
                    let vertex_staging_buffer = vertex_staging_buffer
                        .set(mesh.vertices())
                        .map_err(|error| {
                            shared_state.complete().ok();
                            error
                        })?;
                    let index_staging_buffer =
                        index_staging_buffer.set(mesh.indices()).map_err(|error| {
                            shared_state.complete().ok();
                            error
                        })?;

                    copy_buffers(
                        command_buffer,
                        &mut queue,
                        &mut fence,
                        &[
                            (
                                vertex_staging_buffer,
                                render_mesh.vertex_buffer(),
                                (mesh.vertices().len() * std::mem::size_of::<Vertex>()) as u64,
                            ),
                            (
                                index_staging_buffer,
                                render_mesh.index_buffer(),
                                (mesh.indices().len() * std::mem::size_of::<u32>()) as u64,
                            ),
                        ],
                    )
                    .map_err(|error| {
                        shared_state.complete().ok();
                        error
                    })?;

                    created_objects
                        .send(CreatedRenderObject::Mesh(render_mesh))
                        .ok();

                    shared_state.complete()?;
                }
            }
        }

        Ok(())
    }
}

fn copy_buffers(
    command_buffer: &mut VulkanCommandBuffer,
    queue: &mut VulkanQueue,
    fence: &mut VulkanFence,
    buffers: &[(&VulkanBuffer, &VulkanBuffer, u64)],
) -> Result<()> {
    // Recored the copy commands
    command_buffer.begin().map_err(Error::new_inner)?;
    for (src, dst, size) in buffers {
        command_buffer.cmd_copy_buffer(src, dst, &[VulkanBufferCopy::new(0, 0, *size)]);
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
