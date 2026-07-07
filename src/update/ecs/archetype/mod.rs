use components::*;
use std::any::TypeId;

mod components;
mod set;

mod extend;
mod get;
mod new;
mod push;
mod swap_remove;

pub(in crate::update::ecs) use set::*;

/// An archetype in the Entity Component System (ECS) system, which is a collection of entities
/// that share the same set of components
pub(in crate::update::ecs) struct Archetype {
    /// The IDs of the components that are associated with this archetype
    component_ids: Box<[TypeId]>,

    /// The components and their associated data for the entities in this archetype
    components: Box<[Components]>,

    /// The indices of the archetypes that exist from adding a new component to this archetype
    next_archetypes: Vec<(TypeId, usize)>,

    /// The indices of the archetypes that exist from removing a component from this archetype
    prev_archetypes: Vec<(TypeId, usize)>,
}
