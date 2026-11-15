use crate::render::{LitForwardMaterial, MaterialId, Materials, UnlitMaterial};
use alexandria::Id;

impl Materials {
    /// Remove a material from the collection
    pub fn remove_material<M: Into<MaterialId>>(&mut self, material: M) {
        match material.into() {
            MaterialId::LitForward(id) => self.lit_forward.remove(id),
            MaterialId::Unlit(id) => self.unlit.remove(id),
        }
    }

    /// Remove a lit forward material from the collection
    pub fn remove_lit_forward_material(&mut self, material: Id<LitForwardMaterial>) {
        self.lit_forward.remove(material);
    }

    /// Remove a lit forward material from the collection
    pub fn remove_unlit_material(&mut self, material: Id<UnlitMaterial>) {
        self.unlit.remove(material);
    }
}
