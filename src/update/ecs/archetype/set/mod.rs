use crate::{
    logging::Logger,
    update::ecs::{Archetype, System},
};
use alexandria::{PackedMap, SlotMap};

mod execute_system;
mod get;
mod index;
mod new;
mod push;
mod register_system;
mod unregister_system;

/// The set of archetypes in the Entity Component System (ECS) system
pub(in crate::update::ecs) struct ArchetypeSet {
    /// The actual archetypes in the ECS system
    archetypes: Vec<Archetype>,

    /// The set of pre-update systems registered with the ECS system
    pre_update_systems: PackedMap<System>,

    /// The set of ad hoc systems registered with the ECS system
    ad_hoc_systems: SlotMap<System>,

    /// The set of post-update systems registered with the ECS system
    post_update_systems: PackedMap<System>,

    /// The set of systems that are run for rendering
    rendering_systems: Vec<System>,

    /// The logger for the ECS system
    logger: Logger,
}
