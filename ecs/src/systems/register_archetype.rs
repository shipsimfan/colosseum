use crate::Systems;
use std::any::TypeId;

impl Systems {
    /// Register a new archetype with all systems
    pub fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {
        for (_, phase) in &mut self.phases {
            phase.register_archetype(archetype, components);
        }
    }
}
