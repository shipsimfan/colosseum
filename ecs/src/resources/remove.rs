use crate::Resources;
use std::any::{Any, TypeId};

impl Resources {
    /// Remove a resource from the ECS system, returning it if it exists
    pub fn remove<Resource: 'static>(&mut self) -> Option<Resource> {
        self.remove_by_type_id(TypeId::of::<Resource>())
            .map(|resource| *resource.downcast::<Resource>().unwrap())
    }

    /// Remove a resource from the ECS system by its type ID, returning it if it exists
    pub fn remove_by_type_id(&mut self, r#type: TypeId) -> Option<Box<dyn Any>> {
        self.get_slot_mut(r#type).and_then(|slot| slot.remove())
    }
}
