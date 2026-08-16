use crate::render::RenderData;

impl RenderData {
    /// Reset the render data for a new frame
    pub fn reset(&mut self) {
        self.current_doubled_index = (self.current_doubled_index + 1) % 2;

        self.doubled_mut().unlit_opaque_renderables_mut().reset();
    }
}
