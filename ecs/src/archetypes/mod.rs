use alexandria::SlotMap;

mod entity;

mod new;

pub use entity::*;

/// The set of archetypes in the ECS system
pub(crate) struct Archetypes {
    /// The set of entities in the ECS system, identified by the (archetype, index) pair
    entities: SlotMap<(usize, usize)>,
}
