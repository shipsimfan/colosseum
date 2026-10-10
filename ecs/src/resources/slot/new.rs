use crate::resources::ResourceSlot;
use std::any::{Any, TypeId};

impl ResourceSlot {
    /// Create a new [`ResourceSlot`]
    pub fn new(r#type: TypeId, resource: Box<dyn Any>) -> ResourceSlot {
        ResourceSlot {
            r#type,
            resource: Some(resource),
        }
    }
}
