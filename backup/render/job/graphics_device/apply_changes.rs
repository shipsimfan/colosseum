use crate::{
    Result,
    render::job::{GraphicsDevice, RenderToken},
};

impl GraphicsDevice {
    /// Apply any changes to the graphics device that have been queued up
    pub fn apply_changes(&mut self) -> Result<RenderToken> {
        if let Some(gpu_transfer_queue) = &mut self.gpu_transfer_queue {
            while gpu_transfer_queue.handle_command(&mut self.queue, false)? {}
        }

        self.render_data[self.render_data_index]
            .apply_render_object_changes(&mut self.render_objects);

        let next_render_data_index = (self.render_data_index + 1) % self.render_data.len();
        let token = RenderToken::new(self.render_data_index);
        self.render_data_index = next_render_data_index;

        Ok(token)
    }
}
