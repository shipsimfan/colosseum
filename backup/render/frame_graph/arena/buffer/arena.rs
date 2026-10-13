use crate::render::frame_graph::{Arena, ArenaBuffer, Arenable};

impl<T: Arenable> ArenaBuffer<T> {
    /// Start a new arena from this buffer
    pub fn arena<'a>(&'a mut self) -> Arena<'a, T::T<'a>> {
        Arena {
            data: unsafe { T::transmute(&mut self.buffer) },
        }
    }
}
