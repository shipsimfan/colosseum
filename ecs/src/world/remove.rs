use crate::World;

impl World {
    /// Remove a resource from the world, returning it if it exists
    pub fn remove_resource<Resource: 'static>(&mut self) -> Option<Resource> {
        self.resources.remove::<Resource>()
    }
}
