use crate::update::{ECS, Entity, ecs::DropType};
use alexandria::Id;
use std::any::TypeId;

impl ECS {
    /// Remove a component from an entity
    pub fn remove_component<T: 'static>(&mut self, entity: Id<Entity>) {
        let type_id = TypeId::of::<T>();

        // Get the current archetype and entity index for the given entity
        let (archetype_index, entity_index) = *self
            .entities
            .get(unsafe { entity.cast() })
            .expect("entity does not exist");

        // Get the new archetype index for the entity after removing the component
        let new_archetype_index = self.archetypes[archetype_index]
            .prev_archetype(type_id)
            .expect("component does not exist in entity's archetype");

        // Insert the entity data into the new archetype
        let [source_archetype, target_archetype] = self
            .archetypes
            .get_disjoint_mut(archetype_index, new_archetype_index);

        let new_entity_index = target_archetype.len();
        target_archetype.push(
            source_archetype
                .get_entity_data(entity_index)
                .filter(|(_, id)| *id != type_id),
        );

        // Update the entity's archetype and index in the ECS
        self.entities[unsafe { entity.cast() }] = (new_archetype_index, new_entity_index);

        // Swap remove the entity from the source archetype
        let swapped_entity = source_archetype.swap_remove(entity_index, DropType::One(type_id));

        // Update the swapped entity's index in the ECS
        if let Some(swapped_entity) = swapped_entity {
            self.entities[unsafe { swapped_entity.cast() }] = (archetype_index, entity_index);
        }
    }
}
