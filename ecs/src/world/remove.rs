use crate::World;
use std::any::{Any, TypeId};

impl World {
    /// Remove a resource from the world, returning it if it exists
    pub fn remove_resource<Resource: 'static>(&mut self) -> Option<Resource> {
        self.resources.remove::<Resource>()
    }

    /// Remove a resource from the world by its type ID, returning it if it exists
    pub fn remove_resource_by_type_id(&mut self, r#type: TypeId) -> Option<Box<dyn Any>> {
        self.resources.remove_by_type_id(r#type)
    }
}
