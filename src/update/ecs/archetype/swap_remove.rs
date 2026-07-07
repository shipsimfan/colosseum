use crate::update::{Entity, ecs::Archetype};
use alexandria::Id;

impl Archetype {
    /// Remove an entity from the archetype using the swap-remove method, returning the ID of the entity that was swapped into the removed entity's position, if any
    pub fn swap_remove(&mut self, entity_index: usize) -> Option<Id<Entity>> {
        debug_assert!(
            entity_index < self.components[0].len(),
            "entity index out of bounds"
        );

        let swapped_id = if entity_index < self.components[0].len() - 1 {
            Some(*self.components[0].get(self.components[0].len() - 1))
        } else {
            None
        };

        for component in &mut self.components {
            component.swap_remove(entity_index);
        }

        swapped_id
    }
}
