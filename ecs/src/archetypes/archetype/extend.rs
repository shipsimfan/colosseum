use crate::archetypes::{Archetype, Components};

impl Archetype {
    /// Extend this archetype with a new component type, returning a new archetype
    pub fn extend(&self, new_component: Components) -> Archetype {
        let mut components = Vec::with_capacity(self.components.len() + 1);
        for component in &self.components {
            components.push(component.clone_empty());
        }
        components.push(new_component);

        Archetype::new(components)
    }
}
