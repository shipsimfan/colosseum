use crate::{
    Result,
    render::job::{GraphicsDevice, graphics_device::PerFrameData},
};

impl GraphicsDevice {
    /// Make sure at least `num` command buffers are allocated in the command pool
    pub fn reserve_command_buffers(&mut self, num: usize) -> Result<()> {
        if self.frame_data.len() >= num {
            return Ok(());
        }

        for _ in self.frame_data.len()..num {
            self.frame_data.push(PerFrameData::new(
                &self.render_objects,
                &mut self.command_pool,
                &self.memory_properties,
                &self.device,
            )?);
        }

        Ok(())
    }
}
