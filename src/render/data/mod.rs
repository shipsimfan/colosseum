use crate::render::Material;
use alexandria::Id;

mod material_change;
mod skybox;

mod add;
mod get;
mod new;
mod reset;
mod scene_reset;

pub use skybox::*;

pub(crate) use material_change::*;

/// The data required to execute a render job
pub(crate) struct RenderData {
    /// The skybox to render
    pub skybox: Skybox,

    /// The changes to the set of materials in the scene
    material_changes: Vec<MaterialChange>,

    /// The set of renderable objects in the scene
    renderables: Vec<Id<Material>>,
}
