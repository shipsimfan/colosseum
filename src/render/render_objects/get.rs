use crate::render::{Material, Mesh, RenderMaterial, RenderMesh, RenderObjects};
use alexandria::Id;

impl RenderObjects {
    /// Get the mesh with the given ID
    pub fn mesh(&self, id: Id<Mesh>) -> &RenderMesh {
        &self.meshes[unsafe { id.cast() }]
    }

    /// Get the unlit opaque material with the given ID
    pub fn unlit_opaque_material(&self, id: Id<Material>) -> &RenderMaterial {
        &self.unlit_opaque_materials[unsafe { id.cast() }]
    }
}
