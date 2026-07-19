use crate::{
    Error, GlobalSharedState, Result,
    render::{
        GpuTransferQueue, Vertex,
        transfer::{GpuTransferCommand, StagingBuffer},
    },
};
use alexandria::gpu::{
    VulkanBufferCopy, VulkanCommandBufferLevel, VulkanCommandBufferSubmitInfo,
    VulkanCommandPoolCreateFlag, VulkanDevice, VulkanQueue, VulkanSubmitInfo,
};
use std::sync::mpsc::Receiver;

const INITIAL_STAGING_BUFFER_CAPACITY: usize = 64;

impl GpuTransferQueue {
    pub(in crate::render::transfer) fn thread(
        shared_state: &GlobalSharedState,
        receiver: Receiver<GpuTransferCommand>,
        device: VulkanDevice,
        mut queue: VulkanQueue,
        staging_memory_type: usize,
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
                    vertex_buffer,
                    index_buffer,
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

                    command_buffer.begin().map_err(|error| {
                        shared_state.complete().ok();
                        Error::new_inner(error)
                    })?;
                    command_buffer.cmd_copy_buffer(
                        vertex_staging_buffer,
                        &vertex_buffer,
                        &[VulkanBufferCopy::new(
                            0,
                            0,
                            (mesh.vertices().len() * std::mem::size_of::<Vertex>()) as u64,
                        )],
                    );
                    command_buffer.cmd_copy_buffer(
                        index_staging_buffer,
                        &index_buffer,
                        &[VulkanBufferCopy::new(
                            0,
                            0,
                            (mesh.indices().len() * std::mem::size_of::<u32>()) as u64,
                        )],
                    );
                    command_buffer.end().map_err(|error| {
                        shared_state.complete().ok();
                        Error::new_inner(error)
                    })?;

                    queue
                        .submit(
                            &[VulkanSubmitInfo::new(
                                0,
                                &[],
                                &[VulkanCommandBufferSubmitInfo::new(command_buffer, 0)],
                                &[],
                            )],
                            Some(&mut fence),
                        )
                        .map_err(|error| {
                            shared_state.complete().ok();
                            Error::new_inner(error)
                        })?;

                    fence.wait(u64::MAX).map_err(|error| {
                        shared_state.complete().ok();
                        Error::new_inner(error)
                    })?;

                    shared_state.complete()?;

                    todo!("Transfer the buffers to the render job");
                }
            }
        }

        Ok(())
    }
}
