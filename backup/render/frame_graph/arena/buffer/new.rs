use crate::render::frame_graph::{ArenaBuffer, Arenable};

impl<T: Arenable> ArenaBuffer<T> {
    /// Create a new empty [`ArenaBuffer`]
    pub fn new() -> ArenaBuffer<T> {
        ArenaBuffer { buffer: Vec::new() }
    }
}
