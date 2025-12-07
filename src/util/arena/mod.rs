use slot::Slot;

mod handle;
mod slot;

mod clear;
mod get;
mod index;
mod insert;
mod iter;
mod iter_mut;
mod len;
mod new;
mod remove;

pub use handle::Handle;
pub use iter::ArenaIter;
pub use iter_mut::ArenaIterMut;

/// An arena for holding static objects
pub struct Arena<T: Sized> {
    /// The contained values
    slots: Vec<Slot<T>>,

    /// The number of elements currently in the arena
    len: usize,

    /// The index of the first free slot in the arena, or u32::MAX if there are no free slots
    free_list_head: u32,
}
