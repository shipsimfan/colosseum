use crate::{
    render::{Material, MaterialId, MaterialKind},
    update::UpdateRenderObjects,
};

impl UpdateRenderObjects {
    /// Get a reference to the material with the given ID
    pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
        match id.kind() {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.get(id.id()),
        }
    }

    /// Get a mutable reference to the material with the given ID
    pub fn get_material_mut(&mut self, id: MaterialId) -> Option<&mut Material> {
        match id.kind() {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.get_mut(id.id()),
        }
    }
}
