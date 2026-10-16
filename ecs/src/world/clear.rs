use crate::{PhaseName, World};

impl World {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        self.systems.clear_phase(phase)
    }

    /// Clear all the systems from a single phase using a boxed phase name
    pub fn clear_phase_boxed(&mut self, phase: Box<dyn PhaseName>) {
        self.systems.clear_phase_boxed(phase)
    }
}
