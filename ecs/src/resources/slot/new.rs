use crate::resources::ResourceSlot;
use std::any::{Any, TypeId};

impl ResourceSlot {
    /// Create a new [`ResourceSlot`]
    pub fn new(r#type: TypeId, permanent: bool, resource: Box<dyn Any>) -> ResourceSlot {
        ResourceSlot {
            r#type,
            permanent,
            resource: Some(resource),
        }
    }

    /// Create a new empty [`ResourceSlot`]
    pub fn new_empty(r#type: TypeId, permanent: bool) -> ResourceSlot {
        ResourceSlot {
            r#type,
            permanent,
            resource: None,
        }
    }
}
