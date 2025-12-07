use crate::util::Handle;
use std::marker::PhantomData;

impl<T: Sized> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            index: self.index,
            generation: self.generation,
            _type: PhantomData,
        }
    }
}

impl<T: Sized> Copy for Handle<T> {}
