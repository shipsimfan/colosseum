use crate::update::ecs::archetype::Components;
use std::{alloc::Layout, any::TypeId};

const DEFAULT_CAPACITY: usize = 16;

/// Allocate memory for the initial capacity of a component type
unsafe fn alloc_initial(layout: Layout, element_size: usize) -> *mut u8 {
    let initial_layout =
        Layout::from_size_align(DEFAULT_CAPACITY * element_size, layout.align()).unwrap();

    unsafe { std::alloc::alloc(initial_layout) }
}

impl Components {
    /// Create a new set of component data for a given component type
    pub fn new<T: 'static + Send + Sync + Sized>() -> Components {
        let layout = Layout::new::<T>();
        let element_size = layout.size().next_multiple_of(layout.align());

        let ptr = unsafe { alloc_initial(layout, element_size) };

        Components {
            ptr,
            capacity: DEFAULT_CAPACITY,
            length: 0,
            type_id: TypeId::of::<T>(),
            element_size,
            layout,
            type_name: std::any::type_name::<T>(),
            drop_fn: |ptr| unsafe { std::ptr::drop_in_place(ptr as *mut T) },
        }
    }

    /// Clone the component information without copying the actual data, creating a new set of
    /// component data
    pub fn clone_empty(&self) -> Components {
        let ptr = unsafe { alloc_initial(self.layout, self.element_size) };

        Components {
            ptr,
            capacity: DEFAULT_CAPACITY,
            length: 0,
            type_id: self.type_id,
            element_size: self.element_size,
            layout: self.layout,
            type_name: self.type_name,
            drop_fn: self.drop_fn,
        }
    }
}
