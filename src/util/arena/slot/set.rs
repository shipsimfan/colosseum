use crate::util::arena::Slot;

impl<T: Sized> Slot<T> {
    /// Set the contained value to `value`
    pub fn set(&mut self, value: T) {
        assert!(self.item.is_none());
        self.item = Some(value);
    }
}
