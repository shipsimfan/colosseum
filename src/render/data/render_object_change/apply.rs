use crate::render::{RenderObjectChange, RenderObjects};

impl RenderObjectChange {
    /// Applies the change to the [`Material`]s
    pub(in crate::render) fn apply(self, render_objects: &mut RenderObjects) {
        match self {
            RenderObjectChange::AddMaterial { kind, material } => {
                render_objects.insert_material(kind, material);
            }
            RenderObjectChange::RemoveMaterial { material } => {
                render_objects.remove_material(material);
            }
        }
    }
}
