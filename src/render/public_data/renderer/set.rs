use crate::render::{MaterialId, Mesh, Renderer};
use alexandria::Id;

impl Renderer {
    /// Set the material used to render the object
    pub fn set_material<M: Into<MaterialId>>(&mut self, material: M) -> &mut Renderer {
        self.material = material.into();
        self
    }

    /// Set the mesh used to render the object
    pub fn set_mesh(&mut self, mesh: Id<Mesh>) -> &mut Renderer {
        self.mesh = mesh;
        self
    }
}
