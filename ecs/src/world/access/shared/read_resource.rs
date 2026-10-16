use crate::SharedWorldAccess;
use std::any::TypeId;

impl SharedWorldAccess {
    /// Marks a resource as being read by the system
    pub fn read_resource<Resource: 'static>(&mut self) {
        let r#type = TypeId::of::<Resource>();

        // No need to check if the resource is already being read
        if self.read_resources.contains(&r#type) {
            return;
        }

        // Make sure the resource is not already being written
        if self.write_resources.contains(&r#type) {
            panic!(
                "Resource \"{}\" cannot be read while it is being written by the same system",
                std::any::type_name::<Resource>(),
            );
        }

        // Add the resource to the read list
        self.read_resources.push(r#type);
    }
}
