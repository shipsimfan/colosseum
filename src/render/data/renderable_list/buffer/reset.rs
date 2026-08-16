use crate::render::data::renderable_list::RenderableBuffer;

impl<T> RenderableBuffer<T> {
    /// Reset the renderable buffer for a new frame
    pub fn reset(&mut self) {
        self.renderables.clear();
    }
}
