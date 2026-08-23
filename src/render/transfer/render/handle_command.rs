use crate::{
    Error, Result,
    render::{Mesh, RenderGpuTransferQueue, RenderMesh, Vertex, transfer::GpuTransferCommand},
};
use alexandria::gpu::{
    VulkanBuffer, VulkanBufferCopy, VulkanCommandBuffer, VulkanCommandBufferSubmitInfo,
    VulkanFence, VulkanQueue, VulkanSubmitInfo,
};

impl RenderGpuTransferQueue {
    /// Handle a single command from the transfer queue, returning `true` if a command was handled
    pub fn handle_command(&mut self, queue: &mut VulkanQueue, block: bool) -> Result<bool> {
        let command = if block {
            match self.receiver.recv() {
                Ok(command) => command,
                Err(_) => return Ok(false),
            }
        } else {
            match self.receiver.try_recv() {
                Ok(command) => command,
                Err(_) => return Ok(false),
            }
        };

        match command {
            GpuTransferCommand::Mesh {
                mesh,
                render_mesh,
                allocation,
                sender,
            } => {
                self.transfer_mesh(&mesh, &render_mesh, queue)?;
                sender.send((mesh, render_mesh, allocation))?;
            }
        }

        Ok(true)
    }

    /// Transfer a mesh to the GPU
    pub(in crate::render::transfer::render) fn transfer_mesh(
        &mut self,
        mesh: &Mesh,
        render_mesh: &RenderMesh,
        queue: &mut VulkanQueue,
    ) -> Result<()> {
        let command_buffer = &mut self.command_pool[self.command_buffer_id];

        let vertex_staging_buffer = self.vertex_staging_buffer.set(mesh.vertices())?;
        let index_staging_buffer = self.index_staging_buffer.set(mesh.indices())?;

        copy_buffers(
            command_buffer,
            queue,
            &mut self.fence,
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
        )
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
    fence.reset().map_err(Error::new_inner)?;

    Ok(())
}
