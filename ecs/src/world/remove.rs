use crate::World;

impl World {
    /// Remove a resource from the world, returning it if it exists
    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.resources.remove::<T>()
    }
}
