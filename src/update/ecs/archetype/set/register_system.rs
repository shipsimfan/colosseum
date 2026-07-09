use crate::update::{
    SystemId, SystemPhase,
    ecs::{ArchetypeSet, System},
};

impl ArchetypeSet {
    /// Register a new ad hoc system with the ECS
    pub fn register_system(&mut self, phase: SystemPhase, mut system: System) -> SystemId {
        self.register_all_archetypes(&mut system);
        let id = match phase {
            SystemPhase::PreUpdate => self.pre_update_systems.insert(system),
            SystemPhase::AdHoc => self.ad_hoc_systems.insert(system),
            SystemPhase::PostUpdate => self.post_update_systems.insert(system),
        };
        SystemId::new(phase, id)
    }

    /// Register all archetypes with a new system
    fn register_all_archetypes(&self, system: &mut System) {
        for (index, archetype) in self.archetypes.iter().enumerate() {
            system.register_archetype(index, archetype);
        }
    }
}
