use crate::{UnsafeWorldCell, World};
use std::{marker::PhantomData, ptr::NonNull};

impl World {
    /// Get an unsafe cell for the current [`World`] instance
    pub(in crate::world) fn unsafe_cell<'world>(&'world self) -> UnsafeWorldCell<'world> {
        UnsafeWorldCell {
            world: NonNull::from_ref(self),
            _marker: PhantomData,
        }
    }
}
