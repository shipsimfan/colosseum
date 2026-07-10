use crate::render::{Material, MaterialKind};
use alexandria::Id;

mod display;
mod get;
mod new;

/// The identifier for a material
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId {
    /// The kind of material this is
    kind: MaterialKind,

    /// The id of the material in the list
    id: Id<Material>,
}
