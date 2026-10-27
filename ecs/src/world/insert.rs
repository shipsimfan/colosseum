use crate::World;

impl World {
    /// Insert a new resource into the world, returning if there was already a resource of the same type
    pub fn insert_resource<Resource: 'static>(&mut self, resource: Resource) -> bool {
        self.resources.insert(resource, false)
    }

    /// Insert a new resource into the world, returning if there was already a resource of the same type
    pub fn insert_permanent_resource<Resource: 'static>(&mut self, resource: Resource) -> bool {
        self.resources.insert(resource, true)
    }

    /// Replace an existing resource in the world, returning the old resource if it existed
    pub fn replace_resource<Resource: 'static>(&mut self, resource: Resource) -> Option<Resource> {
        self.resources.replace(resource, false)
    }

    /// Replace an existing resource in the world, returning the old resource if it existed
    pub fn replace_permanent_resource<Resource: 'static>(
        &mut self,
        resource: Resource,
    ) -> Option<Resource> {
        self.resources.replace(resource, true)
    }
}
