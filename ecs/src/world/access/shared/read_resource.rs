use crate::SharedWorldAccess;
use std::any::TypeId;

impl SharedWorldAccess {
    /// Marks a resource as being read by the system
    pub(in crate::world::access) fn read_resource<Resource: 'static>(&mut self) {
        let resource = TypeId::of::<Resource>();

        // No need to check or add if the resource is already being read
        if self.read_resources.contains(&resource) {
            return;
        }

        // Make sure the resource is not already being written
        if self.write_resources.contains(&resource) {
            panic!(
                "Resource \"{}\" cannot be read while it is being written by the same system",
                std::any::type_name::<Resource>(),
            );
        }

        // Add the resource to the read list
        self.read_resources.push(resource);
    }
}
