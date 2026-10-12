use crate::{Archetypes, Resources, Systems};
use colosseum_core::Result;
use std::any::TypeId;

impl Systems {
    /// Run a specific phase of systems
    pub fn run<Phase: 'static>(
        &mut self,
        resources: &mut Resources,
        archetypes: &mut Archetypes,
    ) -> Result<()> {
        if let Some(phase) = self.get_phase_mut(TypeId::of::<Phase>()) {
            phase.run(resources, archetypes)?;
        }

        Ok(())
    }
}
