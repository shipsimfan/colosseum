use crate::ResMut;
use std::ops::{Deref, DerefMut};

impl<'a, Resource: 'static> Deref for ResMut<'a, Resource> {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        self.resource.downcast_ref().unwrap()
    }
}

impl<'a, Resource: 'static> DerefMut for ResMut<'a, Resource> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.resource.downcast_mut().unwrap()
    }
}
