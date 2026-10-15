use crate::Resources;
use std::any::TypeId;

impl Resources {
    /// Remove a resource from the ECS system, returning it if it exists
    pub fn remove<Resource: 'static>(&mut self) -> Option<Resource> {
        self.get_slot_mut(TypeId::of::<Resource>())
            .and_then(|slot| slot.remove())
            .map(|resource| *resource.into_inner().downcast::<Resource>().unwrap())
    }
}
