use crate::{Res, ResMut, World};

impl World {
    /// Get a reference to a resource in the world, if it exists
    pub fn try_get_resource<'a, Resource: 'static>(&'a self) -> Option<Res<'a, Resource>> {
        self.resources.get::<Resource>()
    }

    /// Get a reference to a resource in the world, panicking if it does not exist
    pub fn get_resource<'a, Resource: 'static>(&'a self) -> Res<'a, Resource> {
        self.try_get_resource::<Resource>().unwrap_or_else(|| {
            panic!(
                "Resource of type \"{}\" not found",
                std::any::type_name::<Resource>()
            )
        })
    }

    /// Get a mutable reference to a resource in the world, if it exists
    pub fn try_get_resource_mut<'a, Resource: 'static>(
        &'a mut self,
    ) -> Option<ResMut<'a, Resource>> {
        self.resources.get_mut::<Resource>()
    }

    /// Get a mutable reference to a resource in the world, panicking if it does not exist
    pub fn get_resource_mut<'a, Resource: 'static>(&'a mut self) -> ResMut<'a, Resource> {
        self.try_get_resource_mut::<Resource>().unwrap_or_else(|| {
            panic!(
                "Resource of type \"{}\" not found",
                std::any::type_name::<Resource>()
            )
        })
    }
}
