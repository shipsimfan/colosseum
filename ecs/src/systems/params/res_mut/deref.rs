use crate::ResMut;
use std::ops::{Deref, DerefMut};

impl<'a, Resource: 'static> Deref for ResMut<'a, Resource> {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        self.resource.downcast_ref().unwrap()
    }
}

impl<'a, Resource: 'static> AsRef<Resource> for ResMut<'a, Resource> {
    fn as_ref(&self) -> &Resource {
        self.resource.downcast_ref().unwrap()
    }
}

impl<'a, Resource: 'static> DerefMut for ResMut<'a, Resource> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.resource.downcast_mut().unwrap()
    }
}

impl<'a, Resource: 'static> AsMut<Resource> for ResMut<'a, Resource> {
    fn as_mut(&mut self) -> &mut Resource {
        self.resource.downcast_mut().unwrap()
    }
}
