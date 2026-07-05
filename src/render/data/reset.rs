use crate::render::RenderData;

impl RenderData {
    /// Reset the render data for a new frame
    pub fn reset(&mut self) {
        self.renderables.clear();
    }
}
