use crate::World;

impl World {
    /// Insert a new resource into the world, returning if there was already a resource of the same type
    pub fn insert_resource<T: 'static>(&mut self, resource: T) -> bool {
        self.resources.insert(resource)
    }

    /// Replace an existing resource in the world, returning the old resource if it existed
    pub fn replace_resource<T: 'static>(&mut self, resource: T) -> Option<T> {
        self.resources.replace(resource)
    }
}
