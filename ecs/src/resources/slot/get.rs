use crate::resources::ResourceSlot;
use std::any::{Any, TypeId};

impl ResourceSlot {
    /// Is this resource slot of the specified type?
    pub fn is_type(&self, r#type: TypeId) -> bool {
        self.r#type == r#type
    }

    /// Get a reference to the resource stored in this slot
    pub fn get(&self) -> Option<&Box<dyn Any>> {
        self.resource.as_ref()
    }

    /// Get a mutable reference to the resource stored in this slot
    pub fn get_mut(&mut self) -> Option<&mut Box<dyn Any>> {
        self.resource.as_mut()
    }
}
