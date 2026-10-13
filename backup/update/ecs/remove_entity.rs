use crate::{
    debug,
    update::{ECS, Entity, ecs::DropType},
};
use alexandria::Id;

impl ECS {
    /// Remove an entity from the ECS system
    pub fn remove_entity(&mut self, entity: Id<Entity>) {
        // Remove the entity from the entities map and retrieve its archetype and entity IDs
        let (archetype_index, entity_index) = match self.entities.remove(unsafe { entity.cast() }) {
            Some(entity) => entity,
            None => return,
        };

        // Remove the entity from the corresponding archetype
        let swapped_id = self.archetypes[archetype_index].swap_remove(entity_index, DropType::All);

        // Adjust the swapped id of the entity that was moved to fill the gap left by the removed entity
        if let Some(swapped_id) = swapped_id {
            self.entities[unsafe { swapped_id.cast() }] = (archetype_index, entity_index);
        }

        debug!(self.logger, "Removed entity: {}", entity);
    }
}
