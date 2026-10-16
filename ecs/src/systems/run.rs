use crate::{PhaseName, Systems, UnsafeWorldCell};
use colosseum_core::Result;

impl Systems {
    /// Run a specific phase of systems
    pub fn run<Phase: PhaseName>(&mut self, phase: Phase, world: UnsafeWorldCell) -> Result<()> {
        if let Some(phase) = self.get_phase_mut(&phase) {
            phase.run(world)?;
        }

        Ok(())
    }
}
