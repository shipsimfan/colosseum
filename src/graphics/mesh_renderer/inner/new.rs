use crate::graphics::{Material, Mesh, MeshRendererInner};

impl MeshRendererInner {
    /// Create a new [`MeshRendererInner`]
    pub(in crate::graphics::mesh_renderer) fn new(material: Material, mesh: Mesh) -> Self {
        MeshRendererInner {
            active: true,
            material,
            mesh,
        }
    }
}
