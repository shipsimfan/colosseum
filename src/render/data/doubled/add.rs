use crate::render::{Material, Mesh, ObjectData, data::DoubledRenderData};
use alexandria::Id;

impl DoubledRenderData {
    /// Add a new unlit opaque renderable object to the render data
    pub fn add_unlit_opaque_renderable(
        &mut self,
        material: Id<Material>,
        mesh: Id<Mesh>,
        object: ObjectData,
    ) {
        let address = self.object_buffer.push(object);
        self.unlit_opaque_renderables
            .push((material, mesh, address));
    }

    /// Add a new lit opaque renderable object to the render data
    pub fn add_lit_opaque_renderable(
        &mut self,
        material: Id<Material>,
        mesh: Id<Mesh>,
        object: ObjectData,
    ) {
        let address = self.object_buffer.push(object);
        self.lit_opaque_renderables.push((material, mesh, address));
    }
}
