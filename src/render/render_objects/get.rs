use crate::render::{Material, RenderMaterial, RenderObjects};
use alexandria::Id;

impl RenderObjects {
    /// Get the unlit opaque material with the given ID
    pub(in crate::render) fn unlit_opaque_material(&self, id: Id<Material>) -> &RenderMaterial {
        &self.unlit_opaque_materials[unsafe { id.cast() }]
    }
}
