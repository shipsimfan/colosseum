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
        self.doubled_mut()
            .add_unlit_opaque_renderable(material, mesh, object)
    }

    /// Add a new lit opaque renderable object to the render data
    pub fn add_lit_opaque_renderable(
        &mut self,
        material: Id<Material>,
        mesh: Id<Mesh>,
        object: ObjectData,
    ) {
        self.doubled_mut()
            .add_lit_opaque_renderable(material, mesh, object)
    }
}
