use crate::util::{Arena, Handle};

impl<T: Sized> Arena<T> {
    /// Get a reference to the item at `handle`
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.slots
            .get(handle.index() as usize)?
            .get(handle.generation())
    }

    /// Get a mutable reference to the item at `handle`
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.slots
            .get_mut(handle.index() as usize)?
            .get_mut(handle.generation())
    }
}
