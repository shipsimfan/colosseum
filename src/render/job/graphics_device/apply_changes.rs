use crate::render::{RenderData, job::GraphicsDevice};

impl GraphicsDevice {
    /// Apply any changes to the graphics device that have been queued up
    pub fn apply_changes(&mut self, render_data: &mut RenderData) {
        render_data
            .material_changes()
            .for_each(|change| change.apply(&mut self.materials));
    }
}
