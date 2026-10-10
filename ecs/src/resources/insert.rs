use crate::{Resources, resources::ResourceSlot};
use std::any::{Any, TypeId};

impl Resources {
    /// Insert a new resource to the set, returning if there was already a resource of the same type
    pub fn insert<T: 'static>(&mut self, resource: T) -> bool {
        self.replace(resource).is_some()
    }

    /// Replace an existing resource with a new one, returning the old resource if it existed
    pub fn replace<T: 'static>(&mut self, resource: T) -> Option<T> {
        let r#type = TypeId::of::<T>();
        let resource: Box<dyn Any> = Box::new(resource);

        match self.get_slot_mut(r#type) {
            Some(slot) => slot
                .replace(resource)
                .map(|boxed| *boxed.downcast().unwrap()),
            None => {
                self.resources.push(ResourceSlot::new(r#type, resource));
                None
            }
        }
    }
}
