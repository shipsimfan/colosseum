use crate::{Archetypes, Entity, archetypes::DropType};
use alexandria::Id;
use colosseum_core::as_bytes;
use std::any::TypeId;

impl Archetypes {
    /// Add a component to an entity
    pub fn add_component<Component: 'static + Send + Sync + Sized>(
        &mut self,
        entity: Id<Entity>,
        component: Component,
    ) -> Option<(usize, &[TypeId])> {
        let type_id = TypeId::of::<Component>();

        // Get the current archetype and entity index for the given entity
        let (archetype_index, entity_index) = *self
            .entities
            .get(unsafe { entity.cast() })
            .expect("entity does not exist");

        // Get the new archetype index for the entity after adding the component
        let (new_archetype_index, archetype_created) = self.archetypes[archetype_index]
            .next_archetype(type_id)
            .map(|index| (index, false))
            .unwrap_or_else(|| (self.push::<Component>(archetype_index), true));

        // Insert the component into the new archetype
        let [source_archetype, target_archetype] = self
            .archetypes
            .get_disjoint_mut([archetype_index, new_archetype_index])
            .unwrap();

        let new_entity_index = target_archetype.len();
        target_archetype.push(
            source_archetype
                .get_entity_data(entity_index)
                .chain(std::iter::once((as_bytes(&component), type_id))),
        );
        std::mem::forget(component);

        // Update the entity's archetype and index in the ECS
        self.entities[unsafe { entity.cast() }] = (new_archetype_index, new_entity_index);

        // Swap remove the entity from the source archetype
        let swapped_entity = source_archetype.swap_remove(entity_index, DropType::None);

        // Update the swapped entity's index in the ECS
        if let Some(swapped_entity) = swapped_entity {
            self.entities[unsafe { swapped_entity.cast() }] = (archetype_index, entity_index);
        }

        if archetype_created {
            Some((new_archetype_index, target_archetype.component_ids()))
        } else {
            None
        }
    }
}
