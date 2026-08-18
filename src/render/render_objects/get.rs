use crate::render::{
    FixedRenderObjects, Material, MaterialId, MaterialKind, Mesh, RenderMaterial, RenderMesh,
    RenderObjects,
};
use alexandria::Id;
use std::sync::Arc;

impl RenderObjects {
    /// Get the mesh with the given ID
    pub fn mesh(&self, id: Id<Mesh>) -> &RenderMesh {
        &self.meshes[unsafe { id.cast() }]
    }

    /// Get the unlit opaque material with the given ID
    pub fn unlit_opaque_material(&self, id: Id<Material>) -> &RenderMaterial {
        &self.unlit_opaque_materials[unsafe { id.cast() }]
    }

    /// Get a reference to the material with the given ID
    pub fn material_mut(&mut self, id: MaterialId) -> &mut RenderMaterial {
        match id.kind() {
            MaterialKind::UnlitOpaque => {
                &mut self.unlit_opaque_materials[unsafe { id.id().cast() }]
            }
        }
    }

    /// Get the fixed render objects
    pub fn fixed(&self) -> &Arc<FixedRenderObjects> {
        &self.fixed
    }
}
