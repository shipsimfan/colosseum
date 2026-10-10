use crate::resources::ResourceSlot;
use std::any::Any;

impl ResourceSlot {
    /// Remove the resource from this slot, returning it if it exists
    pub fn remove(&mut self) -> Option<Box<dyn Any>> {
        self.resource.take()
    }
}
