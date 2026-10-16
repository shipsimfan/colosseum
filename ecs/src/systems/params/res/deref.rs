use crate::Res;
use std::ops::Deref;

impl<'a, Resource: 'static> Deref for Res<'a, Resource> {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        self.resource
    }
}

impl<'a, Resource: 'static> AsRef<Resource> for Res<'a, Resource> {
    fn as_ref(&self) -> &Resource {
        self.resource
    }
}
