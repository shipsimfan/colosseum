use crate::render::frame_graph::arena::Arenable;

mod arena;
mod new;

/// The backing buffer for an arena, which is used to store the actual data for the arena
pub(in crate::render::frame_graph) struct ArenaBuffer<T: Arenable> {
    /// The actual buffer storing the data for the arena
    buffer: Vec<T::T<'static>>,
}
