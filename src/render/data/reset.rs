use crate::render::RenderData;

impl RenderData {
    /// Reset the render data for a new frame
    pub fn reset(&mut self, advance: bool) {
        if advance {
            self.current_doubled_index = (self.current_doubled_index + 1) % 2;
        }

        self.doubled_mut().reset();
    }
}
