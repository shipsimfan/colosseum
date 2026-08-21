mod buffer;
mod r#trait;

mod as_slice;
mod deref;
mod drop;
mod index;
mod len;
mod push;

pub(in crate::render::frame_graph) use buffer::*;
pub(in crate::render::frame_graph) use r#trait::*;

/// A vector that can store lifetime elements for a temporary period, while retaining the memory allocated
pub(in crate::render::frame_graph) struct Arena<'a, T> {
    /// The actual vector storing the elements
    data: &'a mut Vec<T>,
}
