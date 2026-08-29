use crate::{Result, render::RenderData};

impl RenderData {
    /// Reserve enough space to store all the renderables
    pub fn reserve_renderables(&mut self, num: usize) -> Result<()> {
        self.doubled[self.current_doubled_index].reserve_renderables(
            num,
            &self.device,
            &self.memory_properties,
        )
    }
}
