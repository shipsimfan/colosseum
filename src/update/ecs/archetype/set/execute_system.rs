use crate::update::{System, ecs::ArchetypeSet};
use alexandria::Id;

impl ArchetypeSet {
    /// Execute an ad hoc system on the archetypes in the ECS system
    pub fn execute_ad_hoc_system(&mut self, system: Id<System>) {
        self.ad_hoc_systems[system].execute(&mut self.archetypes);
    }
}
