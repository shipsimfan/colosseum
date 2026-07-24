use crate::{
    Error, Result,
    render::{
        GpuTransferQueue, Mesh, MeshTransfer, RenderMesh,
        transfer::{GpuTransferCommand, SharedGpuTransferData},
    },
};
use std::sync::Arc;

impl GpuTransferQueue {
    /// Transfer a mesh to the GPU
    pub(in crate::render) fn transfer_mesh(
        &mut self,
        mesh: &Arc<Mesh>,
        render_mesh: RenderMesh,
        index_buffer_offset: u32,
    ) -> Result<MeshTransfer> {
        let shared_state = SharedGpuTransferData::new()?;
        let transfer = MeshTransfer::new(shared_state.clone());

        self.queue
            .send(GpuTransferCommand::Mesh {
                mesh: mesh.clone(),
                shared_state,
                render_mesh,
                index_buffer_offset,
            })
            .map_err(|_| Error::new("transfer queue closed"))?;

        Ok(transfer)
    }
}
