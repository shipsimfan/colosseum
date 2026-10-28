use crate::systems::Phase;
use std::any::TypeId;

impl Phase {
    /// Register an archetype for the phase
    pub fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {
        for &system in &self.archetypes_needed {
            self.systems[system].register_archetype(archetype, components);
        }
    }
}
