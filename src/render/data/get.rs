use crate::render::{Material, MaterialChange, RenderData};
use alexandria::Id;
use std::vec::Drain;

impl RenderData {
    /// Returns a drain iterator over the material changes in the render data
    pub(in crate::render) fn material_changes<'a>(&'a mut self) -> Drain<'a, MaterialChange> {
        self.material_changes.drain(..)
    }

    /// Get the list of renderable objects in the render data
    pub(in crate::render) fn renderables(&self) -> &[Id<Material>] {
        &self.renderables
    }
}
