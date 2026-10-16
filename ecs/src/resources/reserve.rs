use crate::{Resources, resources::ResourceSlot};
use std::any::TypeId;

impl Resources {
    /// Reserve space for a new resource of the given type, if it does not already exist
    pub fn reserve<Resource: 'static>(&mut self) {
        let r#type = TypeId::of::<Resource>();
        if self.get_slot_mut(r#type).is_none() {
            self.resources.push(ResourceSlot::new_empty(r#type));
        }
    }
}
