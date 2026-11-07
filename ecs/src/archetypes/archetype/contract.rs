use crate::archetypes::Archetype;
use std::any::TypeId;

impl Archetype {
    /// Contract this archetype by removing a component type, returning a new archetype
    pub fn contract<Component: 'static + Send + Sync + Sized>(&self) -> Archetype {
        let r#type = TypeId::of::<Component>();

        let mut components = Vec::with_capacity(self.components.len() - 1);
        for component in &self.components {
            if component.type_id() == r#type {
                continue;
            }
            components.push(component.clone_empty());
        }

        debug_assert!(components.len() == self.components.len() - 1);

        Archetype::new(components)
    }
}
