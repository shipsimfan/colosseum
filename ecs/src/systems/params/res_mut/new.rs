use crate::ResMut;
use std::{any::Any, cell::RefMut, marker::PhantomData};

impl<'a, Resource: 'static> ResMut<'a, Resource> {
    /// Create a new [`ResMut`] reference
    pub(crate) fn new(resource: RefMut<'a, Box<dyn Any>>) -> ResMut<'a, Resource> {
        ResMut {
            resource,
            _marker: PhantomData,
        }
    }
}
