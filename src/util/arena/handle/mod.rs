use std::marker::PhantomData;

mod clone;
mod eq;
mod get;
mod new;

/// A handle to an object in an arena
pub struct Handle<T: Sized> {
    /// The index into the arena the handle is good for
    index: u32,

    /// The generation of the slot this handle references
    generation: u32,

    /// The type associated with the arena, to prevent crossing handles
    _type: PhantomData<T>,
}
