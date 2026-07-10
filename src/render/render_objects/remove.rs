use crate::render::{MaterialId, MaterialKind, RenderObjects};

impl RenderObjects {
    /// Remove a material from the render objects
    pub fn remove_material(&mut self, material: MaterialId) {
        match material.kind() {
            MaterialKind::UnlitOpaque => self
                .unlit_opaque_materials
                .remove(unsafe { material.id().cast() }),
        };
    }
}
