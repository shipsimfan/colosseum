use crate::graphics::{MaterialHandle, Mesh, MeshRenderer};

impl MeshRenderer {
    /// Set the material used by this renderer
    pub fn set_material(&mut self, material: MaterialHandle) {
        self.material = material;
    }

    /// Set the mesh used by this renderer
    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;
    }
}
