use crate::render::{MaterialKind, RenderMaterial, RenderObjects};

impl RenderObjects {
    /// Insert a new material into the render objects
    pub fn insert_material(&mut self, kind: MaterialKind, material: RenderMaterial) {
        match kind {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.insert(material),
        };
    }
}
