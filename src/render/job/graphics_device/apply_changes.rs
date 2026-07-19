use crate::render::{CreatedRenderObject, RenderData, job::GraphicsDevice};

impl GraphicsDevice {
    /// Apply any changes to the graphics device that have been queued up
    pub fn apply_changes(&mut self, render_data: &mut RenderData) {
        render_data
            .render_object_changes()
            .for_each(|change| change.apply(&mut self.render_objects));

        while let Ok(created_object) = self.created_objects.try_recv() {
            match created_object {
                CreatedRenderObject::Mesh(mesh) => self.render_objects.insert_mesh(mesh),
            }
        }
    }
}
