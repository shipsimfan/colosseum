use crate::render::{Mesh, RenderMesh, transfer::SharedGpuTransferData};
use std::sync::Arc;

/// Transfer something to the GPU
pub(in crate::render::transfer) enum GpuTransferCommand {
    /// Transfer a mesh to the GPU
    Mesh {
        /// The mesh to be transferred
        mesh: Arc<Mesh>,

        /// The shared state for the transfer
        shared_state: Arc<SharedGpuTransferData>,

        /// The allocated GPU buffers
        render_mesh: RenderMesh,
    },
}
