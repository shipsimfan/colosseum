use crate::Resources;
use std::any::TypeId;

impl Resources {
    /// Remove a resource from the ECS system, returning it if it exists
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.get_slot_mut(TypeId::of::<T>())
            .and_then(|slot| slot.remove())
            .and_then(|resource| *resource.downcast().unwrap())
    }
}
