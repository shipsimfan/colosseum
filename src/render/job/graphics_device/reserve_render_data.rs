use crate::{
    Result,
    render::{RenderData, job::GraphicsDevice},
};

impl GraphicsDevice {
    /// Make sure at least `num` render data structures are allocated
    pub fn reserve_render_data(&mut self, num: usize) -> Result<()> {
        if self.render_data.len() >= num {
            return Ok(());
        }

        for _ in self.render_data.len()..num {
            self.render_data
                .push(RenderData::new(&self.device, &self.memory_properties)?);
        }

        Ok(())
    }
}
