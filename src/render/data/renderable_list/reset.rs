use crate::render::data::RenderableList;

impl<T> RenderableList<T> {
    /// Reset the renderable list for a new frame
    pub fn reset(&mut self) {
        self.count = 0;

        for buffer in &mut self.buffers {
            buffer.reset();
        }
    }
}
