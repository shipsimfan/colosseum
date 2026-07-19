use crate::render::{MaterialKind, RenderMaterial, RenderMesh, RenderObjects};

impl RenderObjects {
    /// Insert a new mesh into the render objects
    pub fn insert_mesh(&mut self, mesh: RenderMesh) {
        self.meshes.insert(mesh);
    }

    /// Insert a new material into the render objects
    pub fn insert_material(&mut self, kind: MaterialKind, material: RenderMaterial) {
        match kind {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.insert(material),
        };
    }
}
