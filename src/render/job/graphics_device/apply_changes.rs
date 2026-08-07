use crate::{
    Result,
    render::{RenderData, job::GraphicsDevice},
};

impl GraphicsDevice {
    /// Apply any changes to the graphics device that have been queued up
    pub fn apply_changes(&mut self, render_data: &mut RenderData) -> Result<()> {
        if let Some(gpu_transfer_queue) = &mut self.gpu_transfer_queue {
            while gpu_transfer_queue.handle_command(&mut self.queue, false)? {}
        }

        render_data.apply_render_object_changes(&mut self.render_objects);
        Ok(())
    }
}
