use crate::update::{ECS, Entity};
use alexandria::Id;

impl ECS {
    /// Get the number of entities in the ECS system
    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }

    /// Get an iterator over the entities in the ECS system
    pub fn entities(&self) -> impl Iterator<Item = Id<Entity>> {
        self.entities
            .key_value_iter()
            .map(|(id, _)| unsafe { id.cast() })
    }

    /// Get a reference to a component of type `T` for the entity with the given `entity_id`
    pub fn get<T: 'static>(&self, entity_id: Id<Entity>) -> &T {
        self.try_get(entity_id)
            .expect("entity does not have the requested component")
    }

    /// Try to get a reference to a component of type `T` for the entity with the given `entity_id`
    pub fn try_get<T: 'static>(&self, entity_id: Id<Entity>) -> Option<&T> {
        let (archetype_index, entity_index) = *self.entities.get(unsafe { entity_id.cast() })?;

        self.archetypes[archetype_index].get::<T>(entity_index)
    }

    /// Get a mutable reference to a component of type `T` for the entity with the given `entity_id`
    pub fn get_mut<T: 'static>(&mut self, entity_id: Id<Entity>) -> &mut T {
        self.try_get_mut(entity_id)
            .expect("entity does not have the requested component")
    }

    /// Try to get a mutable reference to a component of type `T` for the entity with the given `entity_id`
    pub fn try_get_mut<T: 'static>(&mut self, entity_id: Id<Entity>) -> Option<&mut T> {
        let (archetype_index, entity_index) = *self.entities.get(unsafe { entity_id.cast() })?;

        self.archetypes[archetype_index].get_mut::<T>(entity_index)
    }
}
