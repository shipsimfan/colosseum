use crate::render::{Material, RenderData, RenderObjectChange};
use alexandria::Id;
use std::vec::Drain;

impl RenderData {
    /// Returns a drain iterator over the render object changes in the render data
    pub(in crate::render) fn render_object_changes<'a>(
        &'a mut self,
    ) -> Drain<'a, RenderObjectChange> {
        self.render_object_changes.drain(..)
    }

    /// Get the list of unlit opaque renderable objects in the render data
    pub(in crate::render) fn unlit_opaque_renderables(&self) -> &[Id<Material>] {
        &self.unlit_opaque_renderables
    }
}
