use crate::update::ecs::archetype::Components;
use std::{alloc::Layout, any::TypeId, ptr::null_mut};

impl Components {
    /// Create a new set of component data for a given component type
    pub fn new<T: 'static + Send + Sync + Sized>() -> Components {
        Components {
            ptr: null_mut(),
            capacity: 0,
            count: 0,
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            drop_fn: |ptr| unsafe { std::ptr::drop_in_place(ptr as *mut T) },
        }
    }
}
