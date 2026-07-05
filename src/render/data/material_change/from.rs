use crate::render::{Material, MaterialChange, RenderMaterial};
use alexandria::Id;

impl From<RenderMaterial> for MaterialChange {
    fn from(material: RenderMaterial) -> Self {
        MaterialChange::Add(material)
    }
}

impl From<Id<Material>> for MaterialChange {
    fn from(material: Id<Material>) -> Self {
        MaterialChange::Remove(material)
    }
}
