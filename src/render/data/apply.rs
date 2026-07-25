use crate::render::{RenderData, RenderObjects};

impl RenderData {
    /// Apply any render object changes that have been queued up in the render data
    pub(in crate::render) fn apply_render_object_changes(
        &mut self,
        render_objects: &mut RenderObjects,
    ) {
        for change in self.render_object_changes.drain(..) {
            change.apply(render_objects, &mut self.confirmed_removals);
        }
    }
}
