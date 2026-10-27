use crate::archetypes::Components;
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
    pub fn get<Component: 'static>(&self, entity_index: usize) -> &Component {
        debug_assert_eq!(self.type_id, TypeId::of::<Component>());
        debug_assert!(entity_index < self.length);

        let ptr = unsafe { self.ptr.add(entity_index * self.element_size) };
        unsafe { ptr.cast::<Component>().as_ref() }.unwrap()
    }

    /// Get a mutable reference to the component of type `T` for the entity at the given `entity_index`
    pub fn get_mut<Component: 'static>(&mut self, entity_index: usize) -> &mut Component {
        debug_assert_eq!(self.type_id, TypeId::of::<Component>());
        debug_assert!(entity_index < self.length);

        let ptr = unsafe { self.ptr.add(entity_index * self.element_size) };
        unsafe { ptr.cast::<Component>().as_mut() }.unwrap()
    }

    /// Get a reference to the component data of type `T` for all entities in this component data
    pub fn get_all<Component: 'static>(&self) -> &[Component] {
        debug_assert_eq!(self.type_id, TypeId::of::<Component>());

        let ptr = self.ptr.cast::<Component>();
        unsafe { std::slice::from_raw_parts(ptr, self.length) }
    }

    /// Get a mutable reference to the component data of type `T` for all entities in this component data
    pub fn get_all_mut<Component: 'static>(&mut self) -> &mut [Component] {
        debug_assert_eq!(self.type_id, TypeId::of::<Component>());

        let ptr = self.ptr.cast::<Component>();
        unsafe { std::slice::from_raw_parts_mut(ptr, self.length) }
    }

    /// Get the bytes of the component data for the entity at the given `entity_index`
    pub fn get_bytes(&self, entity_index: usize) -> &[u8] {
        debug_assert!(entity_index < self.length);

        let ptr = unsafe { self.ptr.add(entity_index * self.element_size) };
        unsafe { std::slice::from_raw_parts(ptr, self.layout.size()) }
    }
}
