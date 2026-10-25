use crate::{Entity, World};
use alexandria::Id;
use std::any::{Any, TypeId};

impl World {
    /// Remove an entity from the world
    pub fn remove_entity(&mut self, entity: Id<Entity>) {
        self.archetypes.remove_entity(entity);
    }

    /// Remove a component from an entity
    pub fn remove_component<T: 'static>(&mut self, entity: Id<Entity>) {
        self.archetypes.remove_component::<T>(entity);
    }

    /// Remove a resource from the world, returning it if it exists
    pub fn remove_resource<Resource: 'static>(&mut self) -> Option<Resource> {
        self.resources.remove::<Resource>()
    }

    /// Remove a resource from the world by its type ID, returning it if it exists
    pub fn remove_resource_by_type_id(&mut self, r#type: TypeId) -> Option<Box<dyn Any>> {
        self.resources.remove_by_type_id(r#type)
    }
}
