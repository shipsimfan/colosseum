use crate::render::{Material, MaterialId, MaterialKind};
use alexandria::Id;

impl MaterialId {
    /// Create a new [`MaterialId`]
    pub(crate) fn new(kind: MaterialKind, id: Id<Material>) -> MaterialId {
        MaterialId { kind, id }
    }
}
