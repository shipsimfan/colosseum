use crate::update::{System, ecs::ArchetypeSet};
use alexandria::Id;

impl ArchetypeSet {
    /// Register a new ad hoc system with the ECS
    pub fn register_ad_hoc_system(&mut self, mut system: System) -> Id<System> {
        self.register_system(&mut system);
        self.ad_hoc_systems.insert(system)
    }

    /// Register all archetypes with a new system
    fn register_system(&self, system: &mut System) {
        for (index, archetype) in self.archetypes.iter().enumerate() {
            system.register_archetype(index, &archetype.component_ids);
        }
    }
}
