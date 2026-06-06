use crate::render::frame_graph::Arena;

impl<'a, T> Arena<'a, T> {
    /// Get the number of items in the arena
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
