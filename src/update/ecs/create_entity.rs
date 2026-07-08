use crate::{
    debug,
    update::{ECS, Entity, ecs::to_slice},
};
use alexandria::Id;
use std::any::TypeId;

/// The index of the archetype that is used for entities that only have an [`Id<Entity>`] component
const DEFAULT_ARCHETYPE: usize = 0;

impl ECS {
    /// Create a new [`Entity`] in the ECS system with only an [`Id<Entity>`] component
    pub fn create_entity(&mut self) -> Id<Entity> {
        // Create the ID for the new entity
        let index = self.archetypes[DEFAULT_ARCHETYPE].len();
        let id = unsafe { self.entities.insert((DEFAULT_ARCHETYPE, index)).cast() };

        // Insert the entity into the default archetype
        self.archetypes[DEFAULT_ARCHETYPE].push([(to_slice(&id), TypeId::of::<Id<Entity>>())]);

        debug!(self.logger, "Created new entity: {}", id);

        id
    }
}
