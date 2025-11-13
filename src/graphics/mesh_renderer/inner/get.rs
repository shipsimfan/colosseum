use crate::graphics::{Material, Mesh, MeshRendererInner};

impl MeshRendererInner {
    /// Get ther material this renderer uses
    pub fn material(&self) -> &Material {
        &self.material
    }

    /// Get the mesh this renderer uses
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}
