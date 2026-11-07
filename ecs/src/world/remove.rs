use crate::{Entity, World};
use alexandria::Id;

impl World {
    /// Remove an entity from the world
    pub fn remove_entity(&mut self, entity: Id<Entity>) {
        self.archetypes.remove_entity(entity);
    }

    /// Remove a component from an entity
    pub fn remove_component<Component: 'static + Send + Sync + Sized>(
        &mut self,
        entity: Id<Entity>,
    ) {
        if let Some((new_archetype_index, new_archetype_types)) =
            self.archetypes.remove_component::<Component>(entity)
        {
            self.systems
                .register_archetype(new_archetype_index, &new_archetype_types);
        }
    }

    /// Remove a resource from the world, returning it if it exists
    pub fn remove_resource<Resource: 'static>(&mut self) -> Option<Resource> {
        self.resources.remove::<Resource>()
    }
}
