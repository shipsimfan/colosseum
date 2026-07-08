use std::{alloc::Layout, any::TypeId};

mod debug;
mod drop;
mod get;
mod new;
mod push;
mod swap_remove;

/// The data associated with a single component on an archetype in the Entity Component System (ECS) system
pub(in crate::update::ecs) struct Components {
    /// The pointer to the component data
    ptr: *mut u8,

    /// The capacity of the component data, in number of elements
    capacity: usize,

    /// The number of components currently stored in the component data
    length: usize,

    /// The type of the component data
    type_id: TypeId,

    /// The information about the component
    layout: Layout,

    /// The size of a single element of component data, rounded up to the nearest multiple of the alignment of the component data, in bytes
    element_size: usize,

    /// The name of the component type
    type_name: &'static str,

    /// The function to drop the component data
    drop_fn: unsafe fn(*mut u8),
}
