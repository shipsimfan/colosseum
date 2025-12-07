use crate::util::{Arena, Handle};
use std::ops::{Index, IndexMut};

impl<T: Sized> Index<Handle<T>> for Arena<T> {
    type Output = T;

    fn index(&self, index: Handle<T>) -> &Self::Output {
        self.slots[index.index() as usize]
            .get(index.generation())
            .expect("generations don't match at the requested index")
    }
}

impl<T: Sized> IndexMut<Handle<T>> for Arena<T> {
    fn index_mut(&mut self, index: Handle<T>) -> &mut Self::Output {
        self.slots[index.index() as usize]
            .get_mut(index.generation())
            .expect("generations don't match at the requested index")
    }
}
