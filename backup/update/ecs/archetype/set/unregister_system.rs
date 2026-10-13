use crate::update::{SystemId, SystemPhase, ecs::ArchetypeSet};

impl ArchetypeSet {
    /// Unregister a system from the ECS system
    pub fn unregister_system(&mut self, system: SystemId) {
        match system.phase() {
            SystemPhase::PreUpdate => {
                self.pre_update_systems.remove(system.id());
            }
            SystemPhase::AdHoc => {
                self.ad_hoc_systems.remove(system.id());
            }
            SystemPhase::PostUpdate => {
                self.post_update_systems.remove(system.id());
            }
        }
    }
}
