use crate::render::UnlitMaterial;
use alexandria::Id;

mod from;

/// An ID for a material
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialId {
    /// The ID for an unlit material
    Unlit(Id<UnlitMaterial>),
}
