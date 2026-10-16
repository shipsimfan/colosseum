use crate::Commands;
use std::any::TypeId;

impl Commands {
    /// Insert a resource to be added at the end of the phase
    pub fn insert_resource<Resource: 'static>(&mut self, resource: Resource) {
        self.resources_to_insert
            .push((TypeId::of::<Resource>(), Box::new(resource)));
    }
}
