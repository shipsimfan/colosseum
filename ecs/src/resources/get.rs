use crate::{Resources, resources::ResourceSlot};
use std::any::TypeId;

impl Resources {
    /// Get a reference to a resource, if there is one available
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.get_slot(TypeId::of::<T>())
            .and_then(|slot| slot.get())
            .and_then(|resource| resource.downcast_ref())
    }

    /// Get a mutable reference to a resource, if there is one available
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_slot_mut(TypeId::of::<T>())
            .and_then(|slot| slot.get_mut())
            .and_then(|resource| resource.downcast_mut())
    }

    /// Get a reference to a resource slot, if there is one available
    fn get_slot(&self, r#type: TypeId) -> Option<&ResourceSlot> {
        for slot in &self.resources {
            if slot.is_type(r#type) {
                return Some(slot);
            }
        }

        None
    }

    /// Get a mutable reference to a resource slot, if there is one available
    pub(in crate::resources) fn get_slot_mut(
        &mut self,
        r#type: TypeId,
    ) -> Option<&mut ResourceSlot> {
        for slot in &mut self.resources {
            if slot.is_type(r#type) {
                return Some(slot);
            }
        }

        None
    }
}
