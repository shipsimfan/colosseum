use crate::{PhaseName, Systems};

impl Systems {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        if let Some(phase) = self.get_phase_mut(&phase) {
            phase.clear();
        }
    }

    /// Clear all the systems from a single phase using a boxed phase name
    pub fn clear_phase_boxed(&mut self, phase: Box<dyn PhaseName>) {
        if let Some(phase) = self.get_phase_mut(phase.as_ref()) {
            phase.clear();
        }
    }
}
