use crate::resources::ResourceSlot;
use std::{any::Any, cell::RefCell};

impl ResourceSlot {
    /// Replace the contained resource with a new one, returning the old resource if it existed
    pub fn replace(&mut self, resource: RefCell<Box<dyn Any>>) -> Option<RefCell<Box<dyn Any>>> {
        self.resource.replace(resource)
    }
}
