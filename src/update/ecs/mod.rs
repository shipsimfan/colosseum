//! Definitions for the Entity Component System (ECS) system

use crate::logging::Logger;
use alexandria::SlotMap;

mod archetype;
mod entity;
mod system;

mod add_component;
mod create_entity;
mod execute_system;
mod get;
mod new;
mod register_system;
mod remove_component;
mod remove_entity;
mod scene_reset;
mod unregister_system;

pub use archetype::*;
pub use entity::*;
pub use system::{SystemId, SystemPhase};

pub(in crate::update) use system::System;

/// The container for the Entity Component System (ECS) system
pub struct ECS {
    /// The set of entities in the ECS system, identified by the (archetype, entity) pair
    entities: SlotMap<(usize, usize)>,

    /// The set of archetypes in the ECS system
    archetypes: ArchetypeSet,

    logger: Logger,
}

/// Convert a reference to a value of type `T` into a byte slice
fn to_slice<'a, T>(item: &'a T) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts((item as *const T).cast(), std::mem::size_of::<T>()) }
}
