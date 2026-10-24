use crate::render::{Assets, Mesh};
use alexandria::Id;

impl Assets {
    /// Try to get a reference to a mesh by its ID
    pub fn try_get_mesh(&self, id: Id<Mesh>) -> Option<&Mesh> {
        self.meshes.get(id)
    }

    /// Get a reference to a mesh by its ID
    pub fn get_mesh(&self, id: Id<Mesh>) -> &Mesh {
        &self.meshes[id]
    }
}
