use crate::render::{MaterialChange, RenderMaterial};
use alexandria::SlotMap;

impl MaterialChange {
    /// Applies the change to the [`Material`]s
    pub(in crate::render) fn apply(self, materials: &mut SlotMap<RenderMaterial>) {
        match self {
            MaterialChange::Add(material) => {
                materials.insert(material);
            }
            MaterialChange::Remove(id) => {
                materials.remove(unsafe { id.cast() });
            }
        }
    }
}
