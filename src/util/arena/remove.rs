use crate::util::{Arena, Handle};

impl<T: Sized> Arena<T> {
    /// Remove the element at `handle` from this [`Arena`], returning it
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        if handle.index() as usize > self.slots.len() {
            return None;
        }

        let slot = &mut self.slots[handle.index() as usize];
        let result = slot.free(handle.generation(), self.free_list_head);
        if result.is_some() {
            self.len -= 1;
            self.free_list_head = handle.index();
        }
        result
    }
}
