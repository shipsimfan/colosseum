use crate::render::{Material, Mesh, ObjectData, RenderData, RenderObjectChange};
use alexandria::Id;

impl RenderData {
    /// Add a new [`RenderObjectChange`] to the render data
    pub fn add_render_object_change<C: Into<RenderObjectChange>>(&mut self, change: C) {
        self.render_object_changes.push(change.into());
    }

    /// Add a new unlit opaque renderable object to the render data
    pub fn add_unlit_opaque_renderable(
        &mut self,
        material: Id<Material>,
        mesh: Id<Mesh>,
        object: ObjectData,
    ) {
        let index = self.renderable_buffer.push(object);
        self.unlit_opaque_renderables.push((material, mesh, index));
    }

    /// Add a new lit opaque renderable object to the render data
    pub fn add_lit_opaque_renderable(
        &mut self,
        material: Id<Material>,
        mesh: Id<Mesh>,
        object: ObjectData,
    ) {
        let index = self.renderable_buffer.push(object);
        self.lit_opaque_renderables.push((material, mesh, index));
    }
}
