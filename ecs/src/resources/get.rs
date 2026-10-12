use crate::{Res, ResMut, Resources, resources::ResourceSlot};
use std::any::TypeId;

impl Resources {
    /// Get a reference to a resource, if there is one available
    pub fn get<'a, Resource: 'static>(&'a self) -> Option<Res<'a, Resource>> {
        self.get_slot(TypeId::of::<Resource>())
            .and_then(|slot| slot.get())
            .map(Res::new)
    }

    /// Get a mutable reference to a resource, if there is one available
    pub fn get_mut<'a, Resource: 'static>(&'a self) -> Option<ResMut<'a, Resource>> {
        self.get_slot(TypeId::of::<Resource>())
            .and_then(|slot| slot.get_mut())
            .map(ResMut::new)
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
