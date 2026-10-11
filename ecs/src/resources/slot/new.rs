use crate::resources::ResourceSlot;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
};

impl ResourceSlot {
    /// Create a new [`ResourceSlot`]
    pub fn new(r#type: TypeId, resource: RefCell<Box<dyn Any>>) -> ResourceSlot {
        ResourceSlot {
            r#type,
            resource: Some(resource),
        }
    }
}
