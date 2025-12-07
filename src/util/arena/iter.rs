use crate::util::{Arena, arena::Slot};

/// An iterator over the items in an [`Arena`]
pub struct ArenaIter<'a, T: Sized> {
    /// The underlying iterator over the elements
    iter: std::slice::Iter<'a, Slot<T>>,
}

impl<T: Sized> Arena<T> {
    /// Get an iterator over the elements in this [`Arena`]
    pub fn iter<'a>(&'a self) -> ArenaIter<'a, T> {
        ArenaIter {
            iter: self.slots.iter(),
        }
    }
}

impl<'a, T: Sized> IntoIterator for &'a Arena<T> {
    type Item = &'a T;
    type IntoIter = ArenaIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Sized> Iterator for ArenaIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(slot) = self.iter.next() {
            if let Some(element) = slot.get_unchecked() {
                return Some(element);
            }
        }

        None
    }
}
