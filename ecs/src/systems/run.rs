use crate::{Archetypes, PhaseName, Resources, Systems};
use colosseum_core::Result;

impl Systems {
    /// Run a specific phase of systems
    pub fn run<Phase: PhaseName>(
        &mut self,
        phase: Phase,
        resources: &mut Resources,
        archetypes: &mut Archetypes,
    ) -> Result<()> {
        if let Some(phase) = self.get_phase_mut(&phase) {
            phase.run(resources, archetypes)?;
        }

        Ok(())
    }
}
