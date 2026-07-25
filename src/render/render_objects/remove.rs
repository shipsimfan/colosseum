use crate::render::{MaterialId, MaterialKind, Mesh, RenderObjects};
use alexandria::Id;

impl RenderObjects {
    /// Remove a material from the render objects
    pub fn remove_material(&mut self, material: MaterialId) {
        match material.kind() {
            MaterialKind::UnlitOpaque => self
                .unlit_opaque_materials
                .remove(unsafe { material.id().cast() }),
        };
    }

    /// Remove a mesh from the render objects
    pub fn remove_mesh(&mut self, mesh: Id<Mesh>) {
        self.meshes.remove(unsafe { mesh.cast() });
    }
}
