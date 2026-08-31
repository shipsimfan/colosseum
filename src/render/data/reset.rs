use crate::render::RenderData;

impl RenderData {
    /// Reset the render data for a new frame
    pub fn reset(&mut self) {
        self.unlit_opaque_renderables.clear();
        self.lit_opaque_renderables.clear();
        self.renderable_buffer.reset();

        self.lighting.reset();
    }
}
