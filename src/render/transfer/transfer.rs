use crate::{
    Error, Result,
    render::{GpuTransferQueue, Mesh, MeshTransfer, RenderMesh, transfer::GpuTransferCommand},
    single_value_channel,
    update::GpuAllocatedMemory,
};

impl GpuTransferQueue {
    /// Transfer a mesh to the GPU
    pub(in crate::render) fn transfer_mesh(
        &mut self,
        mesh: Mesh,
        render_mesh: RenderMesh,
        allocation: GpuAllocatedMemory,
    ) -> Result<MeshTransfer> {
        let (sender, receiver) = single_value_channel::create(true)?;
        let transfer = MeshTransfer::new(receiver);

        self.queue
            .send(GpuTransferCommand::Mesh {
                mesh,
                render_mesh,
                allocation,
                sender,
            })
            .map_err(|_| Error::new("transfer queue closed"))?;

        Ok(transfer)
    }
}
