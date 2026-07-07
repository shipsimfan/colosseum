use crate::update::ecs::archetype::Components;
use std::any::TypeId;

impl Components {
    /// Get the type ID of the component data
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Get the number of entities in this component data
    pub fn len(&self) -> usize {
        self.length
    }

    /// Get the size of a single component in bytes
    pub fn size(&self) -> usize {
        self.layout.size()
    }

    /// Get a reference to the component of type `T` for the entity at the given `entity_index`
    pub fn get<T: 'static>(&self, entity_index: usize) -> &T {
        debug_assert_eq!(self.type_id, TypeId::of::<T>());
        debug_assert!(entity_index < self.length);

        let ptr = unsafe { self.ptr.add(entity_index * self.element_size) };
        unsafe { ptr.cast::<T>().as_ref() }.unwrap()
    }

    /// Get a mutable reference to the component of type `T` for the entity at the given `entity_index`
    pub fn get_mut<T: 'static>(&mut self, entity_index: usize) -> &mut T {
        debug_assert_eq!(self.type_id, TypeId::of::<T>());
        debug_assert!(entity_index < self.length);

        let ptr = unsafe { self.ptr.add(entity_index * self.element_size) };
        unsafe { ptr.cast::<T>().as_mut() }.unwrap()
    }

    /// Get the bytes of the component data for the entity at the given `entity_index`
    pub fn get_bytes(&self, entity_index: usize) -> &[u8] {
        debug_assert!(entity_index < self.length);

        let ptr = unsafe { self.ptr.add(entity_index * self.element_size) };
        unsafe { std::slice::from_raw_parts(ptr, self.layout.size()) }
    }
}
