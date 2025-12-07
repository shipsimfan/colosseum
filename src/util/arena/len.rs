use crate::util::Arena;

impl<T: Sized> Arena<T> {
    /// Get the number of elements contained in the arena
    pub const fn len(&self) -> usize {
        self.len
    }
}
