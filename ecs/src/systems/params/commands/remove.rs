use crate::Commands;
use std::any::TypeId;

impl Commands {
    /// Remove a resource to be removed at the end of the phase
    pub fn remove_resource<Resource: 'static>(&mut self) {
        self.resources_to_remove.push(TypeId::of::<Resource>());
    }
}
