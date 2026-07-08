use crate::{
    logging::Logger,
    update::{System, ecs::Archetype},
};
use alexandria::PackedMap;

mod execute_system;
mod get;
mod index;
mod new;
mod push;
mod register_system;

/// The set of archetypes in the Entity Component System (ECS) system
pub(in crate::update::ecs) struct ArchetypeSet {
    /// The actual archetypes in the ECS system
    archetypes: Vec<Archetype>,

    /// The set of ad hoc systems registered with the ECS system
    ad_hoc_systems: PackedMap<System>,

    /// The logger for the ECS system
    logger: Logger,
}
