use crate::{PhaseName, Systems};

impl Systems {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        if let Some(phase) = self.get_phase_mut(&phase) {
            phase.clear();
        }
    }
}
