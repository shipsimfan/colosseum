use crate::render::{Materials, UnlitMaterial};
use alexandria::Id;

impl Materials {
    /// Get the ID of the default unlit material
    pub fn default_unlit_material(&self) -> Id<UnlitMaterial> {
        self.default_unlit_material
    }

    /// Try to get a reference to an unlit material by its ID
    pub fn try_get_unlit_material(&self, id: Id<UnlitMaterial>) -> Option<&UnlitMaterial> {
        self.unlit_materials.get(id)
    }

    /// Get a reference to an unlit material by its ID
    pub fn unlit_material(&self, id: Id<UnlitMaterial>) -> &UnlitMaterial {
        &self.unlit_materials[id]
    }

    /// Try to get a mutable reference to an unlit material by its ID
    pub fn try_get_unlit_material_mut(
        &mut self,
        id: Id<UnlitMaterial>,
    ) -> Option<&mut UnlitMaterial> {
        self.unlit_materials.get_mut(id)
    }

    /// Get a mutable reference to an unlit material by its ID
    pub fn unlit_material_mut(&mut self, id: Id<UnlitMaterial>) -> &mut UnlitMaterial {
        &mut self.unlit_materials[id]
    }
}
