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

    /// Get a reference to the component data of type `T` for all entities in this component data
    pub fn get_all<T: 'static>(&self) -> &[T] {
        debug_assert_eq!(self.type_id, TypeId::of::<T>());

        let ptr = self.ptr.cast::<T>();
        unsafe { std::slice::from_raw_parts(ptr, self.length) }
    }

    /// Get a mutable reference to the component data of type `T` for all entities in this component data
    pub fn get_all_mut<T: 'static>(&mut self) -> &mut [T] {
        debug_assert_eq!(self.type_id, TypeId::of::<T>());

        let ptr = self.ptr.cast::<T>();
        unsafe { std::slice::from_raw_parts_mut(ptr, self.length) }
    }

    /// Get the bytes of the component data for the entity at the given `entity_index`
    pub fn get_bytes(&self, entity_index: usize) -> &[u8] {
        debug_assert!(entity_index < self.length);

        let ptr = unsafe { self.ptr.add(entity_index * self.element_size) };
        unsafe { std::slice::from_raw_parts(ptr, self.layout.size()) }
    }
}
