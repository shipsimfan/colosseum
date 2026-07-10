use crate::render::{Material, MaterialId, MaterialKind};
use alexandria::Id;

impl MaterialId {
    /// Get the kind of material this is
    pub(crate) fn kind(&self) -> MaterialKind {
        self.kind
    }

    /// Get the id of the material in the list
    pub(crate) fn id(&self) -> Id<Material> {
        self.id
    }
}
