use crate::render::{MaterialId, UnlitMaterial};
use alexandria::Id;

impl From<Id<UnlitMaterial>> for MaterialId {
    fn from(id: Id<UnlitMaterial>) -> Self {
        MaterialId::Unlit(id)
    }
}
