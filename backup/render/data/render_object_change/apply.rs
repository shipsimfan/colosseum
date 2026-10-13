use crate::render::{RenderObjectChange, RenderObjectRemoveConfirm, RenderObjects};

impl RenderObjectChange {
    /// Applies the change to the [`Material`]s
    pub(in crate::render) fn apply(
        self,
        render_objects: &mut RenderObjects,
        confirmed_removals: &mut Vec<RenderObjectRemoveConfirm>,
    ) {
        match self {
            RenderObjectChange::AddMaterial { kind, material } => {
                render_objects.insert_material(kind, material);
            }
            RenderObjectChange::ChangeMaterialColor { material, color } => {
                render_objects.material_mut(material).set_color(color);
            }
            RenderObjectChange::ChangeSpecularStrength {
                material,
                specular_strength,
            } => {
                render_objects
                    .material_mut(material)
                    .set_specular_strength(specular_strength);
            }
            RenderObjectChange::ChangeShininess {
                material,
                shininess,
            } => {
                render_objects
                    .material_mut(material)
                    .set_shininess(shininess);
            }
            RenderObjectChange::RemoveMaterial { material } => {
                render_objects.remove_material(material);
            }

            RenderObjectChange::AddMesh { mesh } => render_objects.insert_mesh(mesh),
            RenderObjectChange::RemoveMesh { mesh, memory } => {
                render_objects.remove_mesh(mesh);
                confirmed_removals.push(RenderObjectRemoveConfirm::Mesh(memory));
            }
        }
    }
}
