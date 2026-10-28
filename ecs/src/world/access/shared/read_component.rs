use crate::SharedWorldAccess;
use std::any::TypeId;

impl SharedWorldAccess {
    /// Mark a component as being read by the system
    pub(in crate::world::access) fn read_component<Component: 'static>(&mut self) {
        let component = TypeId::of::<Component>();

        // No need to check or add if the component is already being read
        if self.read_components.contains(&component) {
            return;
        }

        // Make sure the component is not already being written
        if self.write_components.contains(&component) {
            panic!(
                "Component \"{}\" cannot be read while it is being written by the same system",
                std::any::type_name::<Component>(),
            );
        }

        self.read_components.push(component);
    }
}
