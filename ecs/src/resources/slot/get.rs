use crate::resources::ResourceSlot;
use std::any::{Any, TypeId};

impl ResourceSlot {
    /// Is this resource slot of the specified type?
    pub fn is_type(&self, r#type: TypeId) -> bool {
        self.r#type == r#type
    }

    /// Get a reference to the resource stored in this slot
    pub fn get<'a>(&'a self) -> Option<&'a Box<dyn Any>> {
        self.resource.as_ref()
    }

    /// Get a mutable reference to the resource stored in this slot
    pub fn get_mut<'a>(&'a mut self) -> Option<&'a mut Box<dyn Any>> {
        self.resource.as_mut()
    }

    /// Is this resource slot permanent?
    pub fn permanent(&self) -> bool {
        self.permanent
    }
}
