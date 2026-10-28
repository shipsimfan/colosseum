use crate::SharedWorldAccess;
use std::any::TypeId;

impl SharedWorldAccess {
    /// Marks a resource as being written by the system
    pub(in crate::world::access) fn write_resource<Resource: 'static>(&mut self) {
        let resource = TypeId::of::<Resource>();

        // Make sure the resource is not already being written
        if self.write_resources.contains(&resource) {
            panic!(
                "Resource \"{}\" cannot be written twice by the same system",
                std::any::type_name::<Resource>(),
            );
        }

        // Make sure the resource is not already being read
        if self.read_resources.contains(&resource) {
            panic!(
                "Resource \"{}\" cannot be written while it is being read by the same system",
                std::any::type_name::<Resource>(),
            );
        }

        // Add the resource to the write list
        self.write_resources.push(resource);
    }
}
