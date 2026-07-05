use crate::render::{Material, MaterialChange, RenderData};
use alexandria::Id;

impl RenderData {
    /// Add a new [`MaterialChange`] to the render data
    pub fn material_change<C: Into<MaterialChange>>(&mut self, change: C) {
        self.material_changes.push(change.into());
    }

    /// Add a new renderable object to the render data
    pub fn renderable(&mut self, material: Id<Material>) {
        self.renderables.push(material);
    }
}
