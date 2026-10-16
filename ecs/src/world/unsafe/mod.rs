use crate::World;
use std::ptr::NonNull;

mod get;
mod new;

/// Unsafe mutable access to a [`World`] instance
pub struct UnsafeWorldCell {
    /// The pointer to the world
    world: NonNull<World>,
}
