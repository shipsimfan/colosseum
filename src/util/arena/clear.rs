use crate::util::{Arena, Handle};

impl<T: Sized> Arena<T> {
    /// Remove all elements in the arena
    pub fn clear(&mut self) {
        for i in 0..self.slots.len() {
            if !self.slots[i].is_free() {
                self.remove(Handle::new(i as _, self.slots[i].generation()));
            }
        }
    }
}
