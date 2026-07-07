use crate::update::{Entity, ecs::Archetype};
use alexandria::Id;
use std::any::TypeId;

impl Archetype {
    /// Get the number of entities in this archetype
    pub fn len(&self) -> usize {
        self.components[0].len()
    }

    /// Get the component IDs of this archetype
    pub fn component_ids(&self) -> &[TypeId] {
        &self.component_ids
    }

    /// Get a component of type `T` for the entity at the given `entity_index`
    pub fn get<T: 'static>(&self, entity_index: usize) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        for components in &self.components {
            if components.type_id() == type_id {
                return Some(components.get(entity_index));
            }
        }

        None
    }

    /// Get a mutable component of type `T` for the entity at the given `entity_index`
    pub fn get_mut<T: 'static>(&mut self, entity_index: usize) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        debug_assert_ne!(
            type_id,
            TypeId::of::<Id<Entity>>(),
            "cannot get mutable reference to Id<Entity> component"
        );

        for components in &mut self.components {
            if components.type_id() == type_id {
                return Some(components.get_mut(entity_index));
            }
        }

        None
    }

    /// Get the set of component data for the entity at the given `entity_index`
    pub fn get_entity_data<'a>(
        &'a self,
        entity_index: usize,
    ) -> impl Iterator<Item = (&'a [u8], TypeId)> {
        self.components
            .iter()
            .map(move |components| (components.get_bytes(entity_index), components.type_id()))
    }

    /// Get the index of the archetype that extends this archetype with the given `type_id`
    pub fn next_archetype(&self, type_id: TypeId) -> Option<usize> {
        #[cfg(debug_assertions)]
        for component_id in &self.component_ids {
            debug_assert_ne!(
                *component_id, type_id,
                "cannot get next archetype for a component that already exists in this archetype"
            );
        }

        for (id, index) in &self.next_archetypes {
            if id == &type_id {
                return Some(*index);
            }
        }

        None
    }

    /// Get the index of the archetype that is this archetype without the given `type_id`
    pub fn previous_archetype(&self, type_id: TypeId) -> Option<usize> {
        debug_assert!(
            self.component_ids.contains(&type_id),
            "cannot get previous archetype for a component that does not exist in this archetype"
        );

        for (id, index) in &self.prev_archetypes {
            if id == &type_id {
                return Some(*index);
            }
        }

        None
    }
}
