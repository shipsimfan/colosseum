use crate::{
    Result,
    render::{GpuTransferQueue, LoadingMesh, Mesh, transfer::GpuTransferCommand},
};
use colosseum_core::{new_error, single_value_channel};

impl GpuTransferQueue {
    /// Transfer a mesh to the GPU
    pub(in crate::render) fn transfer_mesh(&self, mesh: Mesh) -> Result<LoadingMesh> {
        let (sender, receiver) = single_value_channel(true)?;
        let transfer = LoadingMesh::new(receiver);

        self.queue
            .send(GpuTransferCommand::Mesh { mesh, sender })
            .map_err(|_| new_error!("unable to load mesh - GPU transfer queue closed"))?;

        Ok(transfer)
    }
}
