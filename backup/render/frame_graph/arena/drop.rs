use crate::render::frame_graph::Arena;

impl<'a, T> Drop for Arena<'a, T> {
    fn drop(&mut self) {
        self.data.clear();
    }
}
