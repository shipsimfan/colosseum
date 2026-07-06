use crate::update::ecs::archetype::Components;
use std::any::TypeId;

impl Components {
    /// Get the type ID of the component data
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}
