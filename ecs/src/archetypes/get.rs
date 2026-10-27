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
    pub fn get<Component: 'static>(&self, entity_id: Id<Entity>) -> Option<&Component> {
        let (archetype_index, entity_index) = *self.entities.get(unsafe { entity_id.cast() })?;

        self.archetypes[archetype_index].get::<Component>(entity_index)
    }

    /// Try to get a mutable reference to a component of type `T` for the entity with the given `entity_id`
    pub fn get_mut<Component: 'static>(&mut self, entity_id: Id<Entity>) -> Option<&mut Component> {
        let (archetype_index, entity_index) = *self.entities.get(unsafe { entity_id.cast() })?;

        self.archetypes[archetype_index].get_mut::<Component>(entity_index)
    }
}
