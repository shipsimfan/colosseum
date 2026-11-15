use crate::render::{LitForwardMaterial, public_data::materials::LitForwardMaterials};
use alexandria::Id;

impl LitForwardMaterials {
    /// Remove a lit forward material from the collection
    pub fn remove(&mut self, material: Id<LitForwardMaterial>) {
        self.materials.remove(material);
    }
}
