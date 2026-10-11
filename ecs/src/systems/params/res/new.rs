use crate::Res;
use std::{any::Any, cell::Ref, marker::PhantomData};

impl<'a, Resource: 'static> Res<'a, Resource> {
    /// Create a new [`Res`] reference
    pub(crate) fn new(resource: Ref<'a, Box<dyn Any>>) -> Res<'a, Resource> {
        Res {
            resource,
            _marker: PhantomData,
        }
    }
}
