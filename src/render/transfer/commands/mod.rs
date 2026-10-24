use crate::render::Mesh;
use colosseum_core::SingleValueSender;

/// Transfer something to the GPU
pub(in crate::render::transfer) enum GpuTransferCommand {
    /// Transfer a mesh to the GPU
    Mesh {
        /// The mesh to be transferred
        mesh: Mesh,

        /// The sender for the completion state
        sender: SingleValueSender<Mesh>,
    },
}
