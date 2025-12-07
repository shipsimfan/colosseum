use crate::util::{Arena, Handle, arena::Slot};

impl<T: Sized> Arena<T> {
    /// Insert `item` into this arena, returning the handle to it
    pub fn insert(&mut self, item: T) -> Handle<T> {
        self.len += 1;
        if self.free_list_head == u32::MAX {
            self.slots.push(Slot::new(item));
            Handle::new(self.slots.len() as u32 - 1, 0)
        } else {
            let i = self.free_list_head;
            let slot = &mut self.slots[i as usize];
            self.free_list_head = slot.next_free();
            slot.set(item);
            Handle::new(i, slot.generation())
        }
    }
}
