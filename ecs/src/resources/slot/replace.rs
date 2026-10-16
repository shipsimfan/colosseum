use crate::resources::ResourceSlot;
use std::any::Any;

impl ResourceSlot {
    /// Replace the contained resource with a new one, returning the old resource if it existed
    pub fn replace(&mut self, resource: Box<dyn Any>) -> Option<Box<dyn Any>> {
        self.resource.replace(resource)
    }
}
