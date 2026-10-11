use std::{any::Any, cell::Ref, marker::PhantomData};

mod deref;
mod new;

/// An immutable reference to a resource in the ECS
pub struct Res<'a, Resource: 'static> {
    /// The immutable reference to the resource
    resource: Ref<'a, Box<dyn Any>>,

    /// The type-erased resource contained within the `Ref`
    _marker: PhantomData<Resource>,
}
