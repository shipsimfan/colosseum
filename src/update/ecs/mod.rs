use alexandria::SlotMap;
use archetype::*;

mod archetype;
mod entity;

mod new;
mod scene_reset;

pub use entity::*;

/// The container for the Entity Component System (ECS) system
pub(in crate::update) struct ECS {
    /// The set of entities in the ECS system, identified by the (archetype, entity) pair
    entities: SlotMap<(usize, usize)>,

    /// The set of archetypes in the ECS system
    archetypes: ArchetypeSet,
}
