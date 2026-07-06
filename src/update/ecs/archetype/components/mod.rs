use std::{alloc::Layout, any::TypeId};

mod drop;
mod get;
mod new;

/// The data associated with a single component on an archetype in the Entity Component System (ECS) system
pub(in crate::update::ecs::archetype) struct Components {
    /// The pointer to the component data
    ptr: *mut u8,

    /// The capacity of the component data
    capacity: usize,

    /// The number of components currently stored in the component data
    count: usize,

    /// The type of the component data
    type_id: TypeId,

    /// The information about the component
    layout: Layout,

    /// The function to drop the component data
    drop_fn: unsafe fn(*mut u8),
}
