use crate::update::{ECS, System};
use alexandria::Id;

impl ECS {
    /// Register a new ad hoc system with the ECS
    pub fn register_ad_hoc_system(&mut self, system: System) -> Id<System> {
        self.archetypes.register_ad_hoc_system(system)
    }
}
