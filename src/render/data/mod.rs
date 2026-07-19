use crate::render::{Material, Mesh};
use alexandria::Id;

mod render_object_change;
mod skybox;

mod add;
mod get;
mod new;
mod reset;
mod scene_reset;

pub use skybox::*;

pub(crate) use render_object_change::*;

/// The data required to execute a render job
pub(crate) struct RenderData {
    /// The skybox to render
    pub skybox: Skybox,

    /// The changes to the render objects in use
    render_object_changes: Vec<RenderObjectChange>,

    /// The set of unlit opaque renderable objects in the scene
    ///
    /// These renderables are rendered in a single pass, and do not require any lighting
    /// calculations or transparency
    unlit_opaque_renderables: Vec<(Id<Material>, Id<Mesh>)>,
}
