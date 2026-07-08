use crate::update::{ECS, System};
use alexandria::Id;

impl ECS {
    /// Execute an ad hoc system on the archetypes in the ECS system
    pub fn execute_ad_hoc_system(&mut self, system: Id<System>) {
        self.archetypes.execute_ad_hoc_system(system);
    }
}
