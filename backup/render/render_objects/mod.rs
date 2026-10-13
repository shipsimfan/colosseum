use crate::render::{RenderMaterial, RenderMesh};
use alexandria::SlotMap;
use std::sync::Arc;

mod fixed;

mod deref;
mod get;
mod insert;
mod new;
mod remove;

pub(crate) use fixed::*;

/// The render objects that have been created by the update job
pub(in crate::render) struct RenderObjects {
    /// The fixed render objects
    fixed: Arc<FixedRenderObjects>,

    /// The meshes that have been created
    meshes: SlotMap<RenderMesh>,

    /// The unlit opaque materials that have been created
    unlit_opaque_materials: SlotMap<RenderMaterial>,

    /// The lit opaque materials that have been created
    lit_opaque_materials: SlotMap<RenderMaterial>,
}
