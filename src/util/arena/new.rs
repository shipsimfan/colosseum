use crate::util::Arena;

impl<T: Sized> Arena<T> {
    /// Create a new, empty [`Arena`]
    pub const fn new() -> Self {
        Arena {
            slots: Vec::new(),
            len: 0,
            free_list_head: u32::MAX,
        }
    }

    /// Create a new, empty [`Arena`] with at least the specified `capacity`
    pub fn with_capacity(capacity: usize) -> Self {
        Arena {
            slots: Vec::with_capacity(capacity),
            len: 0,
            free_list_head: u32::MAX,
        }
    }
}
