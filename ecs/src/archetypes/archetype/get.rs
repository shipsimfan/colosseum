use crate::{
    Entity,
    archetypes::{Archetype, Components},
};
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

    /// Get the components of this archetype
    pub fn components(&self) -> &[Components] {
        &self.components
    }

    /// Get a component of type `T` for the entity at the given `entity_index`
    pub fn get<Component: 'static>(&self, entity_index: usize) -> Option<&Component> {
        let type_id = TypeId::of::<Component>();

        for components in &self.components {
            if components.type_id() == type_id {
                return Some(components.get(entity_index));
            }
        }

        None
    }

    /// Get a mutable component of type `T` for the entity at the given `entity_index`
    pub fn get_mut<Component: 'static>(&mut self, entity_index: usize) -> Option<&mut Component> {
        let type_id = TypeId::of::<Component>();

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

    /// Get a mutable reference to a set of [`Components`]
    pub fn get_disjoint_components_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> [&mut Components; N] {
        self.components.get_disjoint_mut(indices).unwrap()
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
    pub fn prev_archetype(&self, type_id: TypeId) -> Option<usize> {
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
