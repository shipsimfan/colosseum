use alexandria::SlotMap;
use archetype::*;
use components::*;

mod archetype;
mod components;
mod entity;

mod add_component;
mod create_entity;
mod get;
mod new;
mod push;
mod remove_component;
mod remove_entity;

pub use entity::*;

/// The set of archetypes in the ECS system
pub(crate) struct Archetypes {
    /// The set of entities in the ECS system, identified by the (archetype, index) pair
    entities: SlotMap<(usize, usize)>,

    /// The actual archetypes in the ECS system
    archetypes: Vec<Archetype>,
}
