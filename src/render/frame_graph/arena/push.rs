use crate::render::frame_graph::Arena;

impl<'a, T> Arena<'a, T> {
    /// Push a new value into the arena
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
}
