use crate::{
    SingleValueSender,
    render::{Mesh, RenderMesh},
    update::GpuAllocatedMemory,
};

/// Transfer something to the GPU
pub(in crate::render::transfer) enum GpuTransferCommand {
    /// Transfer a mesh to the GPU
    Mesh {
        /// The mesh to be transferred
        mesh: Mesh,

        /// The allocated GPU buffers
        render_mesh: RenderMesh,

        /// The allocated GPU memory
        allocation: GpuAllocatedMemory,

        /// The sender for the completion state
        sender: SingleValueSender<(Mesh, RenderMesh, GpuAllocatedMemory)>,
    },
}
