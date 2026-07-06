use components::*;
use std::any::TypeId;

mod components;
mod set;

mod new;

pub(in crate::update::ecs) use set::*;

/// An archetype in the Entity Component System (ECS) system, which is a collection of entities
/// that share the same set of components
pub(in crate::update::ecs) struct Archetype {
    /// The IDs of the components that are associated with this archetype
    component_ids: Box<[TypeId]>,

    /// The components and their associated data for the entities in this archetype
    components: Box<[Components]>,
}
