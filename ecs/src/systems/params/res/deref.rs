use crate::Res;
use std::ops::Deref;

impl<'a, Resource: 'static> Deref for Res<'a, Resource> {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        self.resource.downcast_ref().unwrap()
    }
}
