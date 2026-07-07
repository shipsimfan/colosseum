use alexandria::SlotMap;
use archetype::*;

mod archetype;
mod entity;

mod add_component;
mod create_entity;
mod get;
mod new;
mod remove_entity;
mod scene_reset;

pub use entity::*;

/// The container for the Entity Component System (ECS) system
pub struct ECS {
    /// The set of entities in the ECS system, identified by the (archetype, entity) pair
    entities: SlotMap<(usize, usize)>,

    /// The set of archetypes in the ECS system
    archetypes: ArchetypeSet,
}

/// Convert a reference to a value of type `T` into a byte slice
fn to_slice<'a, T>(item: &'a T) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts((item as *const T).cast(), std::mem::size_of::<T>()) }
}
