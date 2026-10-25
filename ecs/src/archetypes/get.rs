use crate::{Archetypes, Entity};
use alexandria::Id;

impl Archetypes {
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

    /// Try to get a reference to a component of type `T` for the entity with the given `entity_id`
    pub fn get<T: 'static>(&self, entity_id: Id<Entity>) -> Option<&T> {
        let (archetype_index, entity_index) = *self.entities.get(unsafe { entity_id.cast() })?;

        self.archetypes[archetype_index].get::<T>(entity_index)
    }

    /// Try to get a mutable reference to a component of type `T` for the entity with the given `entity_id`
    pub fn get_mut<T: 'static>(&mut self, entity_id: Id<Entity>) -> Option<&mut T> {
        let (archetype_index, entity_index) = *self.entities.get(unsafe { entity_id.cast() })?;

        self.archetypes[archetype_index].get_mut::<T>(entity_index)
    }
}
