use crate::{
    Error, Result,
    render::{
        GpuTransferQueue, Mesh, MeshTransfer,
        transfer::{GpuTransferCommand, SharedGpuTransferData},
    },
};
use alexandria::gpu::VulkanBuffer;
use std::sync::Arc;

impl GpuTransferQueue {
    /// Transfer a mesh to the GPU
    pub fn transfer_mesh(
        &mut self,
        mesh: &Arc<Mesh>,
        vertex_buffer: VulkanBuffer,
        index_buffer: VulkanBuffer,
    ) -> Result<MeshTransfer> {
        let shared_state = SharedGpuTransferData::new()?;
        let transfer = MeshTransfer::new(shared_state.clone());

        self.queue
            .send(GpuTransferCommand::Mesh {
                mesh: mesh.clone(),
                shared_state,
                vertex_buffer,
                index_buffer,
            })
            .map_err(|_| Error::new("transfer queue closed"))?;

        Ok(transfer)
    }
}
