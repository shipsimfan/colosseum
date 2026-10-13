use crate::{
    Error, Result,
    render::{Mesh, RenderGpuTransferQueue, RenderMesh, Vertex, transfer::GpuTransferCommand},
};
use alexandria::gpu::{
    VulkanBuffer, VulkanBufferCopy, VulkanCommandBuffer, VulkanCommandBufferSubmitInfo,
    VulkanFence, VulkanQueue, VulkanResult, VulkanSubmitInfo,
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

        let vertices_size = mesh.vertices().len() * std::mem::size_of::<Vertex>();
        let indices_size = mesh.indices().len() * std::mem::size_of::<u32>();
        let size = vertices_size + indices_size;
        self.staging_buffer.resize(size)?;

        self.staging_buffer.set(mesh.vertices(), 0)?;
        let staging_buffer = self.staging_buffer.set(mesh.indices(), vertices_size)?;

        copy_buffers(
            command_buffer,
            queue,
            &mut self.fence,
            [
                (
                    staging_buffer,
                    render_mesh.vertex_buffer(),
                    0,
                    0,
                    vertices_size as _,
                ),
                (
                    staging_buffer,
                    render_mesh.index_buffer(),
                    vertices_size as _,
                    0,
                    indices_size as _,
                ),
            ],
        )
    }
}

fn copy_buffers<const N: usize>(
    command_buffer: &mut VulkanCommandBuffer,
    queue: &mut VulkanQueue,
    fence: &mut VulkanFence,
    buffers: [(&VulkanBuffer, &VulkanBuffer, u64, u64, u64); N],
) -> Result<()> {
    // Recored the copy commands
    command_buffer.begin().map_err(Error::new_inner)?;
    for (src, dst, src_offset, dst_offset, size) in buffers {
        command_buffer.cmd_copy_buffer(
            src,
            dst,
            &[VulkanBufferCopy::new(src_offset, dst_offset, size)],
        );
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
    assert_eq!(
        fence.wait(u64::MAX).map_err(Error::new_inner)?,
        VulkanResult::VkSuccess
    );
    fence.reset().map_err(Error::new_inner)?;

    Ok(())
}
