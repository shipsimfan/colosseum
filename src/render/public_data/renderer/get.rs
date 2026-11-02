use crate::render::{MaterialId, Mesh, Renderer};
use alexandria::Id;

impl Renderer {
    /// Get the ID of the material used to render the object
    pub fn material(&self) -> MaterialId {
        self.material
    }

    /// Get the ID of the mesh used to render the object
    pub fn mesh(&self) -> Id<Mesh> {
        self.mesh
    }
}
