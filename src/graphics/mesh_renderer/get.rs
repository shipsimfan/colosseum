use crate::graphics::{MaterialHandle, Mesh, MeshRenderer};

impl MeshRenderer {
    /// Get ther material this renderer uses
    pub fn material(&self) -> MaterialHandle {
        self.material
    }

    /// Get the mesh this renderer uses
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}
