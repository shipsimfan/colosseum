use crate::{Archetypes, Entity};
use alexandria::Id;
use std::any::TypeId;

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

    /// Get a reference to a specific component at a specific index
    pub fn get_at<Component: 'static>(
        &self,
        archetype: usize,
        index: usize,
        component: usize,
    ) -> &Component {
        self.archetypes[archetype].get_at(index, component)
    }

    /// Get a mutable reference to a specific component at a specific index
    pub fn get_at_mut<Component: 'static>(
        &mut self,
        archetype: usize,
        index: usize,
        component: usize,
    ) -> &mut Component {
        self.archetypes[archetype].get_at_mut(index, component)
    }

    /// Get an iterator over the archetypes in the ECS system
    pub fn archetypes(&self) -> impl Iterator<Item = (usize, &[TypeId])> {
        self.archetypes
            .iter()
            .enumerate()
            .map(|(index, archetype)| (index, archetype.component_ids()))
    }

    /// Get the number of entities in a specific archetype
    pub fn archetype_len(&self, archetype: usize) -> usize {
        self.archetypes[archetype].len()
    }
}
