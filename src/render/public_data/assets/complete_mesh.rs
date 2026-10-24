use crate::render::{Assets, LoadingMesh, Mesh};
use alexandria::Id;

impl Assets {
    /// Complete the transfer of a mesh to the GPU
    pub fn complete_mesh(&mut self, mesh: LoadingMesh) -> Id<Mesh> {
        let mesh = mesh.take();
        self.meshes.insert(mesh)
    }
}
