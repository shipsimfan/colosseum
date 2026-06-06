use crate::render::frame_graph::Arena;

impl<'a, T> Arena<'a, T> {
    /// Get the contents of the arena as a slice
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}
