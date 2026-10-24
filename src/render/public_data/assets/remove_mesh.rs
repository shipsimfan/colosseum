use crate::render::{Assets, Mesh};
use alexandria::Id;

impl Assets {
    /// Remove a mesh by its ID
    pub fn remove_mesh(&mut self, mesh: Id<Mesh>) {
        if let Some(mesh) = self.meshes.remove(mesh) {
            self.mesh_allocator.free(mesh.memory());
        }
    }
}
