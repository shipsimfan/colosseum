use crate::graphics::{Material, MaterialHandle, Materials};

impl Materials {
    /// Get the [`Material`] at `handle`
    pub fn get(&self, handle: MaterialHandle) -> Option<&Material> {
        self.arena.get(handle)
    }

    /// Get the [`Material`] at `handle` mutably
    pub fn get_mut(&mut self, handle: MaterialHandle) -> Option<&mut Material> {
        self.arena.get_mut(handle)
    }

    /// Get the default lit material
    pub fn default_lit(&self) -> MaterialHandle {
        self.default_lit
    }

    /// Get the default unlit material
    pub fn default_unlit(&self) -> MaterialHandle {
        self.default_unlit
    }
}
