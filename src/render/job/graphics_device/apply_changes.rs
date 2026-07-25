use crate::render::{RenderData, job::GraphicsDevice};

impl GraphicsDevice {
    /// Apply any changes to the graphics device that have been queued up
    pub fn apply_changes(&mut self, render_data: &mut RenderData) {
        render_data.apply_render_object_changes(&mut self.render_objects);
    }
}
