use crate::render::{Material, RenderMaterial};
use alexandria::Id;

mod apply;
mod from;

/// The changes to the set of registered materials
pub(crate) enum MaterialChange {
    /// A new material has been added
    Add(RenderMaterial),

    /// A material has been removed
    Remove(Id<Material>),
}
