use std::{any::Any, cell::RefMut, marker::PhantomData};

mod deref;
mod new;
mod system_param;

/// An mutable reference to a resource in the ECS
pub struct ResMut<'a, Resource: 'static> {
    /// The mutable reference to the resource
    resource: RefMut<'a, Box<dyn Any>>,

    /// The type-erased resource contained within the `Ref`
    _marker: PhantomData<Resource>,
}
