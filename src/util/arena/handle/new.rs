use crate::util::Handle;
use std::marker::PhantomData;

impl<T: Sized> Handle<T> {
    /// Create a new [`Handle`] from `index` and `generation`
    pub(in crate::util::arena) fn new(index: u32, generation: u32) -> Self {
        Handle {
            index,
            generation,
            _type: PhantomData,
        }
    }
}
