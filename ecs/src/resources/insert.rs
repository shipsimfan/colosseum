use crate::{Resources, resources::ResourceSlot};
use std::{
    any::{Any, TypeId},
    cell::RefCell,
};

impl Resources {
    /// Insert a new resource to the set, returning if there was already a resource of the same type
    pub fn insert<Resource: 'static>(&mut self, resource: Resource) -> bool {
        self.replace(resource).is_some()
    }

    /// Replace an existing resource with a new one, returning the old resource if it existed
    pub fn replace<Resource: 'static>(&mut self, resource: Resource) -> Option<Resource> {
        let r#type = TypeId::of::<Resource>();
        let resource: RefCell<Box<dyn Any>> = RefCell::new(Box::new(resource));

        match self.get_slot_mut(r#type) {
            Some(slot) => slot
                .replace(resource)
                .map(|boxed| *boxed.into_inner().downcast().unwrap()),
            None => {
                self.resources.push(ResourceSlot::new(r#type, resource));
                None
            }
        }
    }
}
