use crate::util::arena::Slot;

impl<T: Sized> Slot<T> {
    /// Create a new [`Slot`] containing `item`
    pub fn new(item: T) -> Self {
        Slot {
            item: Some(item),
            generation: 0,
            next_free: u32::MAX,
        }
    }
}
