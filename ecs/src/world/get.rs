use crate::{Res, ResMut, World};

impl World {
    /// Get a reference to a resource in the world, if it exists
    pub fn try_get_resource<'a, Resource: 'static>(&'a self) -> Option<Res<'a, Resource>> {
        self.resources.get::<Resource>()
    }

    /// Get a reference to a resource in the world, panicking if it does not exist
    pub fn get_resource<'a, Resource: 'static>(&'a self) -> Res<'a, Resource> {
        self.try_get_resource().unwrap_or_else(|| {
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
        self.resources.get_mut()
    }

    /// Get a mutable reference to a resource in the world, panicking if it does not exist
    pub fn get_resource_mut<'a, Resource: 'static>(&'a mut self) -> ResMut<'a, Resource> {
        self.try_get_resource_mut().unwrap_or_else(|| {
            panic!(
                "Resource of type \"{}\" not found",
                std::any::type_name::<Resource>()
            )
        })
    }

    /// Get the index of a resource, reserving it if it does not already exist
    pub fn get_or_reserve_resource_index<Resource: 'static>(&mut self) -> usize {
        self.resources.get_or_reserve_index::<Resource>(false)
    }

    /// Get the index of a resource, reserving it as a permanent resource if it does not already exist
    pub fn get_or_reserve_permanent_resource_index<Resource: 'static>(&mut self) -> usize {
        self.resources.get_or_reserve_index::<Resource>(true)
    }

    /// Get a reference to a resource in the world by its index, if it exists
    pub unsafe fn try_get_resource_at_index<'a, Resource: 'static>(
        &'a self,
        index: usize,
    ) -> Option<Res<'a, Resource>> {
        unsafe { self.resources.get_at_index(index) }
    }

    /// Get a reference to a resource in the world by its index, panicking if it does not exist
    pub unsafe fn get_resource_at_index<'a, Resource: 'static>(
        &'a self,
        index: usize,
    ) -> Res<'a, Resource> {
        unsafe { self.try_get_resource_at_index(index) }.unwrap_or_else(|| {
            panic!(
                "Resource of type \"{}\" not found at index {}",
                std::any::type_name::<Resource>(),
                index
            )
        })
    }

    /// Get a mutable reference to a resource in the world by its index, if it exists
    pub unsafe fn try_get_resource_at_index_mut<'a, Resource: 'static>(
        &'a mut self,
        index: usize,
    ) -> Option<ResMut<'a, Resource>> {
        unsafe { self.resources.get_at_index_mut(index) }
    }

    /// Get a mutable reference to a resource in the world by its index, panicking if it does not exist
    pub unsafe fn get_resource_at_index_mut<'a, Resource: 'static>(
        &'a mut self,
        index: usize,
    ) -> ResMut<'a, Resource> {
        unsafe { self.try_get_resource_at_index_mut(index) }.unwrap_or_else(|| {
            panic!(
                "Resource of type \"{}\" not found at index {}",
                std::any::type_name::<Resource>(),
                index
            )
        })
    }
}
