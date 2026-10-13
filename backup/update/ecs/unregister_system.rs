use crate::update::{ECS, SystemId};

impl ECS {
    /// Unregister a system from the ECS system
    pub fn unregister_system(&mut self, system: SystemId) {
        self.archetypes.unregister_system(system);
    }
}
