use crate::World;
use std::any::{Any, TypeId};

impl World {
    /// Insert a new resource into the world, returning if there was already a resource of the same type
    pub fn insert_resource<Resource: 'static>(&mut self, resource: Resource) -> bool {
        self.resources.insert(resource)
    }

    /// Insert a new resource into the world by its type ID, returning if there was already a resource of the same type
    pub fn insert_resource_by_type_id(&mut self, r#type: TypeId, resource: Box<dyn Any>) -> bool {
        self.resources.insert_by_type_id(r#type, resource)
    }

    /// Replace an existing resource in the world, returning the old resource if it existed
    pub fn replace_resource<Resource: 'static>(&mut self, resource: Resource) -> Option<Resource> {
        self.resources.replace(resource)
    }
}
