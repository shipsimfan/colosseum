use crate::World;
use std::{cell::UnsafeCell, marker::PhantomData, ptr::NonNull};

mod get;
mod new;

/// Unsafe mutable access to a [`World`] instance
#[derive(Clone)]
pub struct UnsafeWorldCell<'world> {
    /// The pointer to the world
    world: NonNull<World>,

    /// A marker for the lifetime of the borrow
    _marker: PhantomData<(&'world World, &'world UnsafeCell<World>)>,
}
