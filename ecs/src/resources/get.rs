use crate::{Res, ResMut, Resources, resources::ResourceSlot};
use std::any::TypeId;

impl Resources {
    /// Get a reference to a resource, if there is one available
    pub fn get<'a, Resource: 'static>(&'a self) -> Option<Res<'a, Resource>> {
        self.get_slot(TypeId::of::<Resource>())
            .and_then(|slot| slot.get())
            .and_then(|resource| resource.downcast_ref())
            .map(Res::new)
    }

    /// Get a mutable reference to a resource, if there is one available
    pub fn get_mut<'a, Resource: 'static>(&'a mut self) -> Option<ResMut<'a, Resource>> {
        self.get_slot_mut(TypeId::of::<Resource>())
            .and_then(|slot| slot.get_mut())
            .and_then(|resource| resource.downcast_mut())
            .map(ResMut::new)
    }

    /// Get a resource at an index, if it exists
    pub unsafe fn get_at_index<'a, Resource: 'static>(
        &'a self,
        index: usize,
    ) -> Option<Res<'a, Resource>> {
        self.resources
            .get(index)
            .and_then(|slot| slot.get())
            .and_then(|resource| resource.downcast_ref())
            .map(Res::new)
    }

    /// Get a mutable reference to a resource at an index, if it exists
    pub unsafe fn get_at_index_mut<'a, Resource: 'static>(
        &'a mut self,
        index: usize,
    ) -> Option<ResMut<'a, Resource>> {
        self.resources
            .get_mut(index)
            .and_then(|slot| slot.get_mut())
            .and_then(|resource| resource.downcast_mut())
            .map(ResMut::new)
    }

    /// Get the index of a resource, if there is one available
    pub fn get_index<Resource: 'static>(&self) -> Option<usize> {
        for (index, slot) in self.resources.iter().enumerate() {
            if slot.is_type(TypeId::of::<Resource>()) {
                return Some(index);
            }
        }

        None
    }

    /// Get the index of a resource, reserving it if it does not already exist
    pub fn get_or_reserve_index<Resource: 'static>(&mut self) -> usize {
        if let Some(index) = self.get_index::<Resource>() {
            index
        } else {
            self.reserve::<Resource>();
            self.get_index::<Resource>().unwrap()
        }
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
