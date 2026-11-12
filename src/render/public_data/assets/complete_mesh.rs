use crate::{
    Result,
    render::{Assets, LoadingMesh, Mesh},
};
use alexandria::Id;

impl Assets {
    /// Complete the transfer of a mesh to the GPU
    pub fn complete_mesh(&mut self, mesh: LoadingMesh) -> Result<Id<Mesh>> {
        Ok(self.meshes.insert(mesh.take()?))
    }
}
