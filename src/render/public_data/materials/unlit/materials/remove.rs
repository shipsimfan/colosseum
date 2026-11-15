use crate::render::{UnlitMaterial, public_data::materials::UnlitMaterials};
use alexandria::Id;

impl UnlitMaterials {
    /// Remove an unlit material from the collection
    pub fn remove(&mut self, material: Id<UnlitMaterial>) {
        self.materials.remove(material);
    }
}
