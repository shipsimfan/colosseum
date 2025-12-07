use crate::util::{Arena, arena::Slot};

/// An  iterator over the items in an [`Arena`], returning mutable references
pub struct ArenaIterMut<'a, T: Sized> {
    /// The underlying iterator over the elements
    iter: std::slice::IterMut<'a, Slot<T>>,
}

impl<T: Sized> Arena<T> {
    /// Get an iterator over the elements in this [`Arena`]
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIterMut<'a, T> {
        ArenaIterMut {
            iter: self.slots.iter_mut(),
        }
    }
}

impl<'a, T: Sized> IntoIterator for &'a mut Arena<T> {
    type Item = &'a mut T;
    type IntoIter = ArenaIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T: Sized> Iterator for ArenaIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(slot) = self.iter.next() {
            if let Some(element) = slot.get_unchecked_mut() {
                return Some(element);
            }
        }

        None
    }
}
