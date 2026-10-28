use crate::SharedWorldAccess;
use std::any::TypeId;

impl SharedWorldAccess {
    /// Does this system access archetypes?
    pub(in crate::world::access) fn archetypes_needed(&self) -> bool {
        !self.read_components.is_empty() || !self.write_components.is_empty()
    }

    /// Is a resource accessed by this system?
    pub(in crate::world::access) fn resource<Resource: 'static>(&self) -> bool {
        let r#type = TypeId::of::<Resource>();
        self.read_resources.contains(&r#type) || self.write_resources.contains(&r#type)
    }
}
