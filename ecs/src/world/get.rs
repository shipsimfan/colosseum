use crate::World;

impl World {
    /// Get a reference to a resource in the world, if it exists
    pub fn try_get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    /// Get a reference to a resource in the world, panicking if it does not exist
    pub fn get_resource<T: 'static>(&self) -> &T {
        self.try_get_resource::<T>().expect("Resource not found")
    }

    /// Get a mutable reference to a resource in the world, if it exists
    pub fn try_get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// Get a mutable reference to a resource in the world, panicking if it does not exist
    pub fn get_resource_mut<T: 'static>(&mut self) -> &mut T {
        self.try_get_resource_mut::<T>()
            .expect("Resource not found")
    }
}
