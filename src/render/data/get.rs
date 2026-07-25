use crate::render::{Material, Mesh, RenderData, RenderObjectRemoveConfirm};
use alexandria::Id;
use std::vec::Drain;

impl RenderData {
    /// Returns a drain iterator over the confirmed removals in the render data
    pub fn confirmed_removals<'a>(&'a mut self) -> Drain<'a, RenderObjectRemoveConfirm> {
        self.confirmed_removals.drain(..)
    }

    /// Get the list of unlit opaque renderable objects in the render data
    pub(in crate::render) fn unlit_opaque_renderables(&self) -> &[(Id<Material>, Id<Mesh>)] {
        &self.unlit_opaque_renderables
    }
}
