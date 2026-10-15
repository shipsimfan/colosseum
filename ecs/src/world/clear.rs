use crate::{PhaseName, World};

impl World {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        self.systems.clear_phase(phase)
    }
}
