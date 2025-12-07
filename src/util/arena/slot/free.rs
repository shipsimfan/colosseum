use crate::util::arena::Slot;

impl<T: Sized> Slot<T> {
    /// Free the item contained in this slot, if `generation` matches
    pub fn free(&mut self, generation: u32, next_free: u32) -> Option<T> {
        if self.generation != generation || self.item.is_none() {
            return None;
        }

        self.generation += 1;
        self.next_free = next_free;
        self.item.take()
    }
}
