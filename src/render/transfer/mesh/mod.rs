use crate::{
    SingleValueReceiver,
    render::{Mesh, RenderMesh},
    update::GpuAllocatedMemory,
};

mod is_complete;
mod new;
mod take;

/// The state of a mesh transfer
pub struct MeshTransfer {
    /// The receiver for a single value
    receiver: SingleValueReceiver<(Mesh, RenderMesh, GpuAllocatedMemory)>,
}
