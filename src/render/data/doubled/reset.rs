use crate::render::data::DoubledRenderData;

impl DoubledRenderData {
    /// Reset the renderable lists for the new frame
    pub fn reset(&mut self) {
        self.unlit_opaque_renderables.clear();
        self.object_buffer.reset();
    }
}
