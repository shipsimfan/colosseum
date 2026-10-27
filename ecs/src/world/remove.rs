use crate::{Entity, World};
use alexandria::Id;

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
}
