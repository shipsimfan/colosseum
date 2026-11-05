use crate::render::{LitForwardMaterial, MaterialId, UnlitMaterial};
use alexandria::Id;

impl From<Id<UnlitMaterial>> for MaterialId {
    fn from(id: Id<UnlitMaterial>) -> Self {
        MaterialId::Unlit(id)
    }
}

impl From<Id<LitForwardMaterial>> for MaterialId {
    fn from(id: Id<LitForwardMaterial>) -> Self {
        MaterialId::LitForward(id)
    }
}
