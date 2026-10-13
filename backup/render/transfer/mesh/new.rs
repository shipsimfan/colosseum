use crate::{
    SingleValueReceiver,
    render::{Mesh, MeshTransfer, RenderMesh},
    update::GpuAllocatedMemory,
};

impl MeshTransfer {
    /// Create a new [`MeshTransfer`]
    pub(in crate::render::transfer) fn new(
        receiver: SingleValueReceiver<(Mesh, RenderMesh, GpuAllocatedMemory)>,
    ) -> MeshTransfer {
        MeshTransfer { receiver }
    }
}
