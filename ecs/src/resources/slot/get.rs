use crate::resources::ResourceSlot;
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefMut},
};

impl ResourceSlot {
    /// Is this resource slot of the specified type?
    pub fn is_type(&self, r#type: TypeId) -> bool {
        self.r#type == r#type
    }

    /// Get a reference to the resource stored in this slot
    pub fn get<'a>(&'a self) -> Option<Ref<'a, Box<dyn Any>>> {
        self.resource.as_ref().map(|resource| resource.borrow())
    }

    /// Get a mutable reference to the resource stored in this slot
    pub fn get_mut<'a>(&'a self) -> Option<RefMut<'a, Box<dyn Any>>> {
        self.resource.as_ref().map(|resource| resource.borrow_mut())
    }
}
