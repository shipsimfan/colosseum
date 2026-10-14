use crate::{
    render::{Mesh, MeshTransfer, RenderMesh},
    update::GpuAllocatedMemory,
};

impl MeshTransfer {
    /// Take the result of the mesh transfer
    pub(crate) fn take(self) -> (Mesh, RenderMesh, GpuAllocatedMemory) {
        self.receiver.take()
    }
}
