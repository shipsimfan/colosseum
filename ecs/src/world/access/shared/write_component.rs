use crate::SharedWorldAccess;
use std::any::TypeId;

impl SharedWorldAccess {
    /// Mark a component as being written by the system
    pub(in crate::world::access) fn write_component<Component: 'static>(&mut self) {
        let component = TypeId::of::<Component>();

        // Make sure the component is not already being written
        if self.write_components.contains(&component) {
            panic!(
                "Component \"{}\" cannot be written twice by the same system",
                std::any::type_name::<Component>(),
            );
        }

        // Make sure the component is not already being read
        if self.read_components.contains(&component) {
            panic!(
                "Component \"{}\" cannot be written while it is being read by the same system",
                std::any::type_name::<Component>(),
            );
        }

        // Add the component to the write list
        self.write_components.push(component);
    }
}
