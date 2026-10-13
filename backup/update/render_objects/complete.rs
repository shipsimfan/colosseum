use crate::{
    render::Mesh,
    update::{GpuAllocatedMemory, UpdateRenderObjects},
};
use alexandria::Id;

impl UpdateRenderObjects {
    /// Complete the transfer of a mesh to the GPU
    pub fn complete_mesh(&mut self, mesh: Mesh, allocation: GpuAllocatedMemory) -> Id<Mesh> {
        let id = self.meshes.insert((mesh, allocation));
        unsafe { id.cast() }
    }
}
