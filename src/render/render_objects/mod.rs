use crate::render::{RenderMaterial, RenderMesh};
use alexandria::SlotMap;

mod get;
mod insert;
mod new;
mod remove;

/// The render objects that have been created by the update job
pub(in crate::render) struct RenderObjects {
    /// The meshes that have been created
    meshes: SlotMap<RenderMesh>,

    /// The unlit opaque materials that have been created
    unlit_opaque_materials: SlotMap<RenderMaterial>,
}
