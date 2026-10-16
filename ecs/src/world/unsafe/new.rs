use crate::{UnsafeWorldCell, World};
use std::ptr::NonNull;

impl World {
    /// Get an unsafe cell for the current [`World`] instance
    pub(in crate::world) fn unsafe_cell(&self) -> UnsafeWorldCell {
        UnsafeWorldCell {
            world: NonNull::from_ref(self),
        }
    }
}
